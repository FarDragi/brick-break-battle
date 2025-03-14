use crate::components::bar::Bar;
use crate::components::player::Player;
use crate::systems::networking::{NetworkEvent, NetworkingConfig};
use bevy::asset::io::memory::Value::Vec;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{
    ButtonInput, Commands, KeyCode, Query, Res, Sprite, Time, Transform, With, info,
};
use bevy_ggrs::PlayerInputs;
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Bar,
        Sprite {
            color: Color::srgb(1., 0., 0.),
            custom_size: Some(Vec2::new(100., 100.)),
            ..Default::default()
        },
        KinematicCharacterController::default(),
        Collider::cuboid(50., 50.),
        RigidBody::KinematicPositionBased,
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0., 1., 0.),
            custom_size: Some(Vec2::new(20., 20.)),
            ..Default::default()
        },
        Collider::cuboid(10., 10.),
        RigidBody::Dynamic,
        Transform::from_xyz(0., 100., 0.),
    ));
}

pub fn read_player_input(
    button: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    for mut controller in players.iter_mut() {
        let mut aditional_move = Vec2::ZERO;

        if button.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            aditional_move += Vec2::new(-1., 0.);
        }
        if button.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            aditional_move += Vec2::new(1., 0.);
        }

        let position = if let Some(position) = controller.translation {
            position
        } else {
            Vec2::ZERO
        };

        controller.translation = Some(position + (aditional_move * time.delta_secs() * 100.));
    }
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
