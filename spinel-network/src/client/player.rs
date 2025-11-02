use std::{error::Error, io::{self, ErrorKind}, net::SocketAddr, str::FromStr};
use uuid::Uuid;

pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub protocol_version: i32,
    pub addr: SocketAddr,
}

impl Player {
    pub fn new(uuid: Uuid, username: String, protocol_version: i32, addr: SocketAddr) -> Self { 
        Self {
            uuid,
            username,
            protocol_version,
            addr,
        }
    }
}