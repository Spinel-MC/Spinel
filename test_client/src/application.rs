use spinel::client::MinecraftClient;
use spinel::core::network::serverbound::handshake::intention::IntentionPacket;
use spinel::core::network::serverbound::login::login_start::LoginStartPacket;
use spinel::core::network::serverbound::play::player_loaded::PlayerLoadedPacket;
use spinel::core::network::serverbound::play::use_item_on::UseItemOnPacket;
use spinel::network::ConnectionState;
use spinel::network::types::Position;
use spinel::utils::constants::PROTOCOL_VERSION;
use spinel::uuid::Uuid;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::dispatch::report_dispatch_result;

pub struct TestClientApplication {
    client: MinecraftClient,
    server_ip: String,
    player_name: String,
    port: u16,
    should_click_entity_sign: bool,
}

impl TestClientApplication {
    pub fn new() -> Self {
        Self {
            client: MinecraftClient::new(),
            server_ip: configured_ip(),
            player_name: configured_name(),
            port: configured_port(),
            should_click_entity_sign: configured_entity_sign_click(),
        }
    }

    pub async fn run(mut self) {
        println!(
            "Starting Spinel Test Client on {}:{} as {}",
            self.server_ip, self.port, self.player_name
        );
        self.run_join_sequence().await;
    }

    async fn run_join_sequence(&mut self) {
        if self.connect_to_play().await {
            self.send_player_loaded().await;
            self.start_join_automation();
        }

        self.run_command_loop().await;
    }

    async fn connect_to_play(&mut self) -> bool {
        if let Err(error) = self.client.connect(&self.server_ip, self.port).await {
            eprintln!(
                "Failed to connect to {}:{}: {}",
                self.server_ip, self.port, error
            );
            return false;
        }

        self.client.set_state(ConnectionState::Handshaking);
        if !report_dispatch_result(
            IntentionPacket {
                protocol_version: PROTOCOL_VERSION as i32,
                server_address: self.server_ip.clone(),
                server_port: self.port,
                intention: 2,
            }
            .dispatch(&mut self.client),
            "login intention packet",
        ) {
            return false;
        }
        self.client.set_state(ConnectionState::Login);

        if !report_dispatch_result(
            LoginStartPacket {
                name: self.player_name.clone(),
                uuid: Uuid::new_v4(),
            }
            .dispatch(&mut self.client),
            "login start packet",
        ) {
            return false;
        }

        self.wait_for_play_state().await;
        self.client.refresh_state_from_server() == Some(ConnectionState::Play)
    }

    async fn send_player_loaded(&mut self) {
        if self.client.refresh_state_from_server() != Some(ConnectionState::Play) {
            println!("Player loaded packet skipped: client did not reach Play state.");
            return;
        }

        report_dispatch_result(
            PlayerLoadedPacket {}.dispatch(&mut self.client),
            "player loaded packet",
        );
    }

    fn start_join_automation(&self) {
        let mut automation_client = self.client.clone();
        let should_click_entity_sign = self.should_click_entity_sign;

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            if should_click_entity_sign {
                Self::click_entity_sign(&mut automation_client).await;
            }
        });
    }

    async fn click_entity_sign(client: &mut MinecraftClient) {
        if client.refresh_state_from_server() != Some(ConnectionState::Play) {
            println!("Entity sign click skipped: client did not reach Play state.");
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
            .dispatch(client),
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

    async fn run_command_loop(&mut self) {
        println!("Commands: respawn, right_click, swing, left_click <ticks>, reconnect, quit");

        let mut stdin_lines = Some(BufReader::new(tokio::io::stdin()).lines());
        let mut reconnect_interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        reconnect_interval.tick().await;

        loop {
            tokio::select! {
                line = read_next_command(&mut stdin_lines), if stdin_lines.is_some() => {
                    if !self.should_continue_after_command_line(line).await {
                        return;
                    }
                }
                _ = reconnect_interval.tick() => {
                    self.reconnect_if_disconnected().await;
                }
            };
        }
    }

    async fn should_continue_after_command_line(
        &mut self,
        line: std::io::Result<Option<String>>,
    ) -> bool {
        let Ok(Some(command)) = line else {
            return true;
        };

        let mut command_parts = command.split_whitespace();
        let Some(command_name) = command_parts.next() else {
            return true;
        };

        match command_name {
            "respawn" => {
                report_dispatch_result(self.client.respawn(), "respawn command packet");
            }
            "right_click" => {
                report_dispatch_result(self.client.right_click(), "right click packet");
            }
            "swing" => {
                report_dispatch_result(self.client.left_click(), "left click packet");
            }
            "left_click" => {
                let Some(ticks) = parse_left_click_ticks(command_parts.next()) else {
                    println!("Usage: left_click <ticks>");
                    return true;
                };
                self.hold_left_click_for_ticks(ticks).await;
            }
            "reconnect" => {
                self.reconnect().await;
            }
            "quit" => {
                self.client
                    .disconnect_for_reason("test client command loop quit")
                    .await;
                return false;
            }
            "" => {}
            unknown_command => {
                println!("Unknown command: {}", unknown_command);
            }
        }

        true
    }

    async fn hold_left_click_for_ticks(&mut self, ticks: u32) {
        report_dispatch_result(self.client.left_click(), "left click packet");

        for _ in 1..ticks {
            report_dispatch_result(self.client.continue_left_click(), "left click hold packet");
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        report_dispatch_result(
            self.client.release_left_click(),
            "left click release packet",
        );
    }

    async fn reconnect(&mut self) {
        self.client
            .disconnect_for_reason("test client reconnect requested")
            .await;
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        self.reconnect_to_play().await;
    }

    async fn reconnect_if_disconnected(&mut self) {
        if self.client.server_state().is_some() {
            return;
        }

        println!("Disconnected. Reconnecting.");
        self.reconnect_to_play().await;
    }

    async fn reconnect_to_play(&mut self) {
        self.client = MinecraftClient::new();
        if self.connect_to_play().await {
            self.send_player_loaded().await;
        }
    }
}

fn configured_ip() -> String {
    configured_argument("--ip")
        .or_else(|| std::env::var("SPINEL_TEST_CLIENT_IP").ok())
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

fn configured_name() -> String {
    configured_argument("--name")
        .or_else(|| std::env::var("SPINEL_TEST_CLIENT_NAME").ok())
        .unwrap_or_else(|| "Spinel".to_string())
}

fn configured_port() -> u16 {
    configured_argument("--port")
        .or_else(|| std::env::var("SPINEL_TEST_CLIENT_PORT").ok())
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(25565)
}

fn configured_entity_sign_click() -> bool {
    std::env::var("SPINEL_TEST_CLIENT_CLICK_ENTITY_SIGN")
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn configured_argument(flag_name: &str) -> Option<String> {
    let flag_assignment_prefix = format!("{flag_name}=");
    let mut arguments = std::env::args().skip(1);

    while let Some(argument) = arguments.next() {
        if argument == flag_name {
            return arguments.next();
        }

        if let Some(argument_value) = argument.strip_prefix(&flag_assignment_prefix) {
            return Some(argument_value.to_string());
        }
    }

    None
}

async fn read_next_command(
    stdin_lines: &mut Option<tokio::io::Lines<BufReader<tokio::io::Stdin>>>,
) -> std::io::Result<Option<String>> {
    let Some(lines) = stdin_lines else {
        return Ok(None);
    };

    let command = lines.next_line().await?;

    if command.is_none() {
        *stdin_lines = None;
    }

    Ok(command)
}

fn parse_left_click_ticks(argument: Option<&str>) -> Option<u32> {
    argument
        .and_then(|ticks| ticks.parse::<u32>().ok())
        .filter(|ticks| *ticks > 0)
}
