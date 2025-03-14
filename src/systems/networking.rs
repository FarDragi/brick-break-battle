use bevy::prelude::{Commands, ResMut, error, info};
use bevy_ggrs::Session;
use bevy_ggrs::ggrs::Config;
use bevy_ggrs::prelude::SessionBuilder;
use bevy_matchbox::MatchboxSocket;
use bevy_matchbox::prelude::PeerId;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NetworkEvent {
    #[default]
    Nothing,
    Left,
    Right
}

#[derive(Debug)]
pub struct NetworkingConfig;

impl Config for NetworkingConfig {
    type Input = NetworkEvent;
    type State = u8;
    type Address = PeerId;
}

pub fn setup_networking(mut commands: Commands) {
    let room_url = "ws://localhost:3536/room?next=2";
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

pub fn wait_connection(mut command: Commands, mut socket: ResMut<MatchboxSocket>) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();

    for player in players.iter() {
        info!("Player connected: {:?}", player);
    }

    if players.len() != 2 {
        return;
    }

    let mut session_builder = SessionBuilder::<NetworkingConfig>::new()
        .with_num_players(2)
        .with_input_delay(1);

    for (i, player) in players.into_iter().enumerate() {
        let result = session_builder.add_player(player, i);

        if result.is_err() {
            error!("Failed to add player to session: {:?}", result);
            return;
        }

        session_builder = result.unwrap();
    }

    let channel = socket.take_channel(0);

    if channel.is_err() {
        error!("Failed to get channel: {:?}", channel);
        return;
    }

    let ggrs_session = session_builder.start_p2p_session(channel.unwrap());

    if ggrs_session.is_err() {
        error!("Failed to start p2p session");
        return;
    }

    command.insert_resource(Session::P2P(ggrs_session.unwrap()));
}
