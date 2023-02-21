use crate::resources::{LocalPlayerHandle, Session};
use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_ggrs::ggrs::{self, PlayerType};
use matchbox_socket::WebRtcSocket;

pub fn start_matchbox_system(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/game?next=2";
    let (socket, message_loop) = WebRtcSocket::new(room_url);
    IoTaskPool::get().spawn(message_loop).detach();
    commands.insert_resource(Session {
        socket: Some(socket),
    });
}

pub fn wait_for_players(mut session: ResMut<Session>, mut commands: Commands) {
    let Some(socket) = &mut session.socket else {
        // If there is no socket we've already started the game
        return;
    };

    // Check for new connections
    socket.accept_new_connections();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }
    info!("All peers have joined, going in-game");
    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        if player == PlayerType::Local {
            commands.insert_resource(LocalPlayerHandle(i));
        }
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the socket out of the resource (required because GGRS takes ownership of it)
    let socket = session.socket.take().unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(socket)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));
}

pub struct GGRSConfig;
impl ggrs::Config for GGRSConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are strings
    type Address = String;
}
