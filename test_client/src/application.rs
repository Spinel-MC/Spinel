use spinel::client::MinecraftClient;
use spinel::core::network::serverbound::handshake::intention::IntentionPacket;
use spinel::core::network::serverbound::login::login_start::LoginStartPacket;
use spinel::core::network::serverbound::status::ping_request::PingRequestPacket;
use spinel::core::network::serverbound::status::status_request::StatusRequestPacket;
use spinel::network::ConnectionState;
use spinel::utils::constants::PROTOCOL_VERSION;
use spinel::uuid::Uuid;

use crate::dispatch::report_dispatch_result;

pub struct TestClientApplication {
    client: MinecraftClient,
}

impl TestClientApplication {
    pub fn new() -> Self {
        Self {
            client: MinecraftClient::new(),
        }
    }

    pub async fn run(mut self) {
        println!("=== Spinel Test Client ===\n");
        self.run_status_ping().await;
        self.run_join_sequence().await;
    }

    async fn run_status_ping(&mut self) {
        println!("--- STATUS PING START ---");
        self.client.connect("127.0.0.1", 25565).await;
        report_dispatch_result(
            IntentionPacket {
                protocol_version: PROTOCOL_VERSION as i32,
                server_address: "127.0.0.1".to_string(),
                server_port: 25565,
                intention: 1,
            }
            .dispatch(&mut self.client),
            "status intention packet",
        );
        self.client.set_state(ConnectionState::Status);

        println!("Sending Status Request...");
        report_dispatch_result(
            StatusRequestPacket {}.dispatch(&mut self.client),
            "status request packet",
        );

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        println!("Sending Ping Request...");
        report_dispatch_result(
            PingRequestPacket {
                timestamp: 123456789,
            }
            .dispatch(&mut self.client),
            "ping request packet",
        );

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        self.client.disconnect().await;
        println!("--- STATUS PING END ---\n");
    }

    async fn run_join_sequence(&mut self) {
        println!("--- JOIN START ---");
        self.client.connect("127.0.0.1", 25565).await;
        self.client.set_state(ConnectionState::Handshaking);
        report_dispatch_result(
            IntentionPacket {
                protocol_version: PROTOCOL_VERSION as i32,
                server_address: "127.0.0.1".to_string(),
                server_port: 25565,
                intention: 2,
            }
            .dispatch(&mut self.client),
            "login intention packet",
        );
        self.client.set_state(ConnectionState::Login);

        println!("Sending Login Start...");
        report_dispatch_result(
            LoginStartPacket {
                name: "Spinel".to_string(),
                uuid: Uuid::new_v4(),
            }
            .dispatch(&mut self.client),
            "login start packet",
        );

        self.wait_for_play_state().await;
        self.run_fast_movement_probe().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        self.client.disconnect().await;
        println!("--- JOIN END ---");
    }

    async fn wait_for_play_state(&mut self) {
        for _ in 0..200 {
            if self.client.refresh_state_from_server() == Some(ConnectionState::Play) {
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }

    async fn run_fast_movement_probe(&mut self) {
        if self.client.refresh_state_from_server() != Some(ConnectionState::Play) {
            println!("Fast movement probe skipped because the client did not reach play state.");
            return;
        }

        report_dispatch_result(
            self.client.acknowledge_chunk_batch(64.0),
            "initial chunk batch acknowledgement",
        );

        for movement_step in 0..160 {
            report_dispatch_result(
                self.client.move_by(24.0, 0.0, 0.0, true),
                "fast movement packet",
            );

            if movement_step % 4 == 0 {
                report_dispatch_result(
                    self.client.acknowledge_chunk_batch(64.0),
                    "chunk batch acknowledgement",
                );
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        println!("Fast movement probe ended at {:?}", self.client.position());
    }
}
