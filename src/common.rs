use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    pos(f32, f32),
}

#[derive(Serialize, Deserialize)]
pub enum FromServerMessage {
    Pong(usize), // Used for connection oriented protocols
    UnknownPong, // Used for non-connection oriented protocols
    pos(f32, f32),
}