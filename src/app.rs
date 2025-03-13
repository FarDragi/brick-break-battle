use crate::components::bar::Bar;
use crate::components::player::Player;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

pub fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..Default::default()
        },
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1080.,
            },
            ..OrthographicProjection::default_2d()
        }
    ));

    commands.spawn((
        Player,
        Bar,
        Sprite {
            color: Color::srgb(1., 0., 0.),
            custom_size: Some(Vec2::new(100., 100.)),
            ..Default::default()
        },
    ));
}
