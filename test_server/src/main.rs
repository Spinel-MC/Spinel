use test_server::app::TestServer;

#[tokio::main]
async fn main() {
    TestServer::new(server_port()).run().await;
}

fn server_port() -> u16 {
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        if argument != "--port" {
            panic!("Unknown test server argument: {argument}");
        }
        let port = arguments
            .next()
            .expect("The --port option requires a port number");
        return port
            .parse()
            .expect("The --port option must contain a valid u16 port number");
    }
    25560
}
