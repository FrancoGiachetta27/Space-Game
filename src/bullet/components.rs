use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub target_player: bool, // wether the bullet is targeting the player
    pub speed: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct LifeTime {
    pub timer: Timer,
}
