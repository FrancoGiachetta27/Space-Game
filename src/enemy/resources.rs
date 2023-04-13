use bevy::prelude::*;

// Timer to track enemy spawing time
#[derive(Resource)]
pub struct EnemyConfig {
    pub spawning_timer: Timer,
    pub shooting_timer: Timer,
}
