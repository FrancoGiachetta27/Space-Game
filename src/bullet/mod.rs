use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const BULLET_SIZE: Vec2 = Vec2::new(16.0, 16.0);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((bullet_movement, bullet_collition, despawn_bullet));
    }
}
