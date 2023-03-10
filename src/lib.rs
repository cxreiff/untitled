mod config_plugin;
mod kitty_plugin;
mod loading_plugin;

pub use crate::config_plugin::{
    get_world_position, CameraFlag, ConfigPlugin, ASPECT_RATIO, HEIGHT, WIDTH,
};
pub use crate::loading_plugin::{LoadingPlugin, TextureAssets};

use crate::kitty_plugin::KittyPlugin;
use bevy::prelude::{App, Plugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Load,
    Play,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin).add_plugin(KittyPlugin);
    }
}
