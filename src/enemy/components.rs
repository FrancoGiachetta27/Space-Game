use bevy::prelude::*;

use crate::{bullet::components::Bullet, resources::GameAssets};

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

#[derive(Component)]
pub enum EnemyType {
    Normal,
    DobleShooter,
    CircleShooter,
    AimShooter,
}

impl EnemyType {
    pub fn get_enemy(&self, game_assets: &GameAssets) -> (Handle<Image>, Enemy) {
        match self {
            EnemyType::Normal => (
                game_assets.enemy_1.clone(),
                Enemy {
                    health: 50.0,
                    speed: 65.0,
                },
            ),
            EnemyType::DobleShooter => (
                game_assets.enemy_1.clone(),
                Enemy {
                    health: 50.0,
                    speed: 65.0,
                },
            ),
            EnemyType::CircleShooter => (
                game_assets.enemy_3.clone(),
                Enemy {
                    health: 50.0,
                    speed: 65.0,
                },
            ),
            EnemyType::AimShooter => (
                game_assets.enemy_3.clone(),
                Enemy {
                    health: 50.0,
                    speed: 65.0,
                },
            ),
        }
    }
    pub fn get_bullet(&self, game_assets: &GameAssets) -> (Handle<Image>, Bullet) {
        match self {
            EnemyType::Normal => (
                game_assets.bullet.clone(),
                Bullet {
                    target_player: true,
                    speed: 100.0,
                    damage: 5.0,
                },
            ),
            EnemyType::DobleShooter => (
                game_assets.bullet.clone(),
                Bullet {
                    target_player: true,
                    speed: 100.0,
                    damage: 5.0,
                },
            ),
            EnemyType::CircleShooter => (
                game_assets.bullet.clone(),
                Bullet {
                    target_player: true,
                    speed: 100.0,
                    damage: 5.0,
                },
            ),
            EnemyType::AimShooter => (
                game_assets.bullet.clone(),
                Bullet {
                    target_player: true,
                    speed: 100.0,
                    damage: 5.0,
                },
            ),
        }
    }
}
