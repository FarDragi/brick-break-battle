use crate::components::bar::Bar;
use crate::components::player::Player;
use crate::systems::networking::{NetworkEvent, NetworkingConfig};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{ButtonInput, Commands, KeyCode, Query, Res, Sprite, Time, Transform, With};
use bevy::utils::HashMap;
use bevy_ggrs::{AddRollbackCommandExtension, LocalInputs, LocalPlayers, PlayerInputs};

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn((
            Player,
            Bar,
            Sprite {
                color: Color::srgb(1., 0., 0.),
                custom_size: Some(Vec2::new(100., 100.)),
                ..Default::default()
            },
        ))
        .add_rollback();
}

pub fn read_player_input(
    mut commands: Commands,
    button: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        if button.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            local_inputs.insert(*handle, NetworkEvent::Left);
        }
        if button.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            local_inputs.insert(*handle, NetworkEvent::Right);
        }
    }

    commands.insert_resource(LocalInputs::<NetworkingConfig>(local_inputs));
}

pub fn move_player(
    inputs: Res<PlayerInputs<NetworkingConfig>>,
    mut players: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for input in inputs.iter() {
        let direction = match input.0 {
            NetworkEvent::Left => Vec2::new(-1., 0.),
            NetworkEvent::Right => Vec2::new(1., 0.),
            _ => Vec2::ZERO,
        };

        for mut transform in players.iter_mut() {
            transform.translation += (direction * time.delta().as_secs_f32() * 1000.).extend(0.)
        }
    }
}
