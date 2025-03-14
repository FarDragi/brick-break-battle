use crate::systems::networking::{NetworkingConfig, setup_networking, wait_connection};
use crate::systems::player::{move_player, read_player_input, setup_player};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Brick break battle".into(),
                        resolution: (1920., 1080.).into(),
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(GgrsPlugin::<NetworkingConfig>::default())
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.5)))
        .rollback_component_with_clone::<Transform>()
        .add_systems(Startup, (setup, setup_networking, setup_player))
        .add_systems(Update, wait_connection)
        .add_systems(ReadInputs, read_player_input)
        .add_systems(GgrsSchedule, move_player);
    }
}

pub fn setup(mut commands: Commands) {
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
        },
    ));
}
