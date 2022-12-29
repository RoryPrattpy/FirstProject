use bevy::log::LogSettings;
use local_ip_address::local_ip;
use client_test_2::*;
use client_test_2::KeyCode;
use macroquad::prelude::*;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};


static mut Player1: Player = Player {
    pos: [0., 0.],
    velocity: [0., 0.],
    grounded: false,
    dimensions: [25., 25.],
    active: false,
};
static mut Player2: Player = Player {
    pos: [0., 0.],
    velocity: [0., 0.],
    grounded: false,
    dimensions: [25., 25.],
    active: false,
};
static mut Player3: Player = Player {
    pos: [0., 0.],
    velocity: [0., 0.],
    grounded: false,
    dimensions: [25., 25.],
    active: false,
};

struct Player {
    pos: [f32; 2],
    velocity: [f32; 2],
    grounded: bool,
    dimensions: [f32; 2],
    active: bool,
}

fn create_renet_client() -> RenetClient {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let socket = UdpSocket::bind("192.168.86.37:6142").unwrap();

    let client_id = current_time.as_millis() as u64;

    let connection_config = RenetConnectionConfig::default();

    //TODO Prompt for server IP
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 86, 37)), 42069);

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(
        current_time,
        socket,
        client_id,
        connection_config,
        authentication,
    )
    .unwrap()
}
#[macroquad::main("Best App")]
async fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin)
        .insert_resource(create_renet_client())
        .add_system(client_ping)
        .run();
}

fn client_ping(mut client: ResMut<RenetClient>, keyboard: Res<Input<KeyCode>>) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Up) {
        let ping_message = bincode::serialize(&ClientMessage::Up).unwrap();
        client.send_message(reliable_channel_id, ping_message);
        println!("Sent ping!");
    }
    if keyboard.just_pressed(KeyCode::Left) {
        let ping_message = bincode::serialize(&ClientMessage::Left).unwrap();
        client.send_message(reliable_channel_id, ping_message);
        println!("Sent ping!");
    }
    if keyboard.just_pressed(KeyCode::Right) {
        let ping_message = bincode::serialize(&ClientMessage::Right).unwrap();
        client.send_message(reliable_channel_id, ping_message);
        println!("Sent ping!");
    }

    while let Some(message) = client.receive_message(reliable_channel_id) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::Users([p1, p2, p3]) => {
                unsafe {
                    ([Player1.pos[0], Player1.pos[1]], [Player2.pos[0], Player2.pos[1]], [Player3.pos[0], Player3.pos[1]]) = (p1, p2, p3);
                }
            }
        }
    }
}