use crate::systems::networking::{NetworkingConfig, setup_networking, wait_connection};
use crate::systems::player::{move_player, read_player_input, setup_player};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use bevy_rapier2d::plugin::NoUserData;
use bevy_rapier2d::prelude::{RapierDebugRenderPlugin, RapierPhysicsPlugin};

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
        // .add_plugins(GgrsPlugin::<NetworkingConfig>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.5)))
        // .rollback_component_with_clone::<Transform>()
        .add_systems(Startup, (setup, setup_player))
        .add_systems(Update, read_player_input);
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
