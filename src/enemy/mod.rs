use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;

use resources::EnemyConfig;

const ENEMY_SIZE: Vec2 = Vec2::new(48.0, 48.0);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyConfig {
            spawning_timer: Timer::from_seconds(5.5, TimerMode::Repeating),
            shooting_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        })
        .add_systems((spawn_enemy, enemy_shoot, enemy_death))
        .add_systems((avoid_other_enemies, search_player, avoid_bullets).chain());
    }
}
