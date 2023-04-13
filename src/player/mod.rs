use bevy::{input::mouse::MouseButtonInput, prelude::*};

pub mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(movement)
            .add_system(player_shooting.run_if(on_event::<MouseButtonInput>()));
    }
}
