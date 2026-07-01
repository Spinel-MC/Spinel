use crate::server::MinecraftServer;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[tokio::test]
async fn server_start_returns_after_stop() {
    let server = Arc::new(Mutex::new(MinecraftServer::new()));
    let port = available_loopback_port();
    let server_task = tokio::spawn(MinecraftServer::start_shared(
        server.clone(),
        "127.0.0.1",
        port,
    ));

    wait_until_server_accepts_connections(port).await;
    server.lock().unwrap().stop();

    tokio::time::timeout(Duration::from_secs(1), server_task)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn stopped_server_no_longer_accepts_connections() {
    let server = Arc::new(Mutex::new(MinecraftServer::new()));
    let port = available_loopback_port();
    let server_task = tokio::spawn(MinecraftServer::start_shared(
        server.clone(),
        "127.0.0.1",
        port,
    ));

    wait_until_server_accepts_connections(port).await;
    server.lock().unwrap().stop();
    tokio::time::timeout(Duration::from_secs(1), server_task)
        .await
        .unwrap()
        .unwrap();

    let connection_attempt = tokio::time::timeout(
        Duration::from_millis(200),
        tokio::net::TcpStream::connect(("127.0.0.1", port)),
    )
    .await;

    assert!(connection_attempt.is_err() || connection_attempt.unwrap().is_err());
}

#[test]
fn stop_after_current_tick_waits_for_tick_lifecycle_boundary() {
    let mut server = MinecraftServer::new();
    server
        .is_ticking
        .store(true, std::sync::atomic::Ordering::SeqCst);

    server.stop_after_current_tick();

    assert!(server.is_ticking.load(std::sync::atomic::Ordering::SeqCst));
    server.process_lifecycle_request();
    assert!(!server.is_ticking.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn restart_after_current_tick_preserves_restart_request_at_lifecycle_boundary() {
    let mut server = MinecraftServer::new();
    server
        .is_ticking
        .store(true, std::sync::atomic::Ordering::SeqCst);

    server.restart_after_current_tick();

    assert!(server.is_ticking.load(std::sync::atomic::Ordering::SeqCst));
    server.process_lifecycle_request();
    assert!(!server.is_ticking.load(std::sync::atomic::Ordering::SeqCst));
    assert!(server.restart_was_requested());
}
fn available_loopback_port() -> u16 {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).unwrap();
    listener.local_addr().unwrap().port()
}

async fn wait_until_server_accepts_connections(port: u16) {
    let deadline = Instant::now() + Duration::from_secs(1);
    while Instant::now() < deadline {
        if tokio::net::TcpStream::connect(("127.0.0.1", port))
            .await
            .is_ok()
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("server did not accept connections on port {port}");
}
