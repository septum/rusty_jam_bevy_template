use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use rusty_jam_bevy_template::{
    config, game,
    resources::{self, prelude::*},
    scenes,
};

fn main() {
    App::new()
        .add_plugin(config::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(resources::Plugin)
        .add_plugin(scenes::Plugin)
        .insert_resource(ClearColor(Colors::DARK))
        .add_state(game::State::Startup)
        .run();
}
