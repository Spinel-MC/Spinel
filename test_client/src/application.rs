use spinel::client::MinecraftClient;
use spinel::core::network::serverbound::handshake::intention::IntentionPacket;
use spinel::core::network::serverbound::login::login_start::LoginStartPacket;
use spinel::core::network::serverbound::play::player_loaded::PlayerLoadedPacket;
use spinel::core::network::serverbound::play::use_item_on::UseItemOnPacket;
use spinel::core::network::serverbound::status::ping_request::PingRequestPacket;
use spinel::core::network::serverbound::status::status_request::StatusRequestPacket;
use spinel::network::ConnectionState;
use spinel::network::types::Position;
use spinel::utils::constants::PROTOCOL_VERSION;
use spinel::uuid::Uuid;

use crate::dispatch::report_dispatch_result;

pub struct TestClientApplication {
    client: MinecraftClient,
    port: u16,
    should_run_fast_movement_probe: bool,
    should_click_entity_sign: bool,
}

impl TestClientApplication {
    pub fn new() -> Self {
        Self {
            client: MinecraftClient::new(),
            port: configured_port(),
            should_run_fast_movement_probe: configured_fast_movement_probe(),
            should_click_entity_sign: configured_entity_sign_click(),
        }
    }

    pub async fn run(mut self) {
        println!("=== Spinel Test Client ===\n");
        self.run_status_ping().await;
        self.run_join_sequence().await;
    }

    async fn run_status_ping(&mut self) {
        println!("--- STATUS PING START ---");
        self.client.connect("127.0.0.1", self.port).await;
        report_dispatch_result(
            IntentionPacket {
                protocol_version: PROTOCOL_VERSION as i32,
                server_address: "127.0.0.1".to_string(),
                server_port: self.port,
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
        self.client.connect("127.0.0.1", self.port).await;
        self.client.set_state(ConnectionState::Handshaking);
        report_dispatch_result(
            IntentionPacket {
                protocol_version: PROTOCOL_VERSION as i32,
                server_address: "127.0.0.1".to_string(),
                server_port: self.port,
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
        self.send_player_loaded().await;
        if self.should_click_entity_sign {
            self.click_entity_sign().await;
        }
        if self.should_run_fast_movement_probe {
            self.run_fast_movement_probe().await;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        self.client.disconnect().await;
        println!("--- JOIN END ---");
    }

    async fn send_player_loaded(&mut self) {
        if self.client.refresh_state_from_server() != Some(ConnectionState::Play) {
            println!("Player loaded packet skipped because the client did not reach play state.");
            return;
        }

        report_dispatch_result(
            PlayerLoadedPacket {}.dispatch(&mut self.client),
            "player loaded packet",
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    async fn click_entity_sign(&mut self) {
        if self.client.refresh_state_from_server() != Some(ConnectionState::Play) {
            println!("Entity sign click skipped because the client did not reach play state.");
            return;
        }

        report_dispatch_result(
            UseItemOnPacket {
                hand: 0,
                block_position: Position { x: 3, y: 4, z: 5 },
                block_face: 3,
                cursor_position_x: 0.5,
                cursor_position_y: 0.5,
                cursor_position_z: 0.5,
                inside_block: false,
                hit_world_border: false,
                sequence: 1,
            }
            .dispatch(&mut self.client),
            "entity sign use item on packet",
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
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

fn configured_port() -> u16 {
    std::env::var("SPINEL_TEST_CLIENT_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(25565)
}

fn configured_fast_movement_probe() -> bool {
    std::env::var("SPINEL_TEST_CLIENT_FAST_MOVEMENT")
        .map(|value| value != "0")
        .unwrap_or(true)
}

fn configured_entity_sign_click() -> bool {
    std::env::var("SPINEL_TEST_CLIENT_CLICK_ENTITY_SIGN")
        .map(|value| value != "0")
        .unwrap_or(false)
}
