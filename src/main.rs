mod app;
pub mod components;
mod systems;

use crate::app::AppPlugin;
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
