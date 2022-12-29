pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{Deserialize, Serialize};

pub struct Player {
    pos: Vec2,
    velocity: Vec2,
    grounded: bool,
    active: bool,
    dimensions: Vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Left,
    Right,
    Up,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Users([[f32; 2]; 3]),
}

pub const PROTOCOL_ID: u64 = 1000;