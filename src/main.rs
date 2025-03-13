mod app;
pub mod components;
mod systems;

use crate::app::AppPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Brick break battle".into(),
                        resolution: (1920., 1080.).into(),
                        present_mode: PresentMode::AutoNoVsync,
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(AppPlugin)
        .run();
}
