use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use crate::{
    bullet::{
        components::{Bullet, LifeTime},
        BULLET_SIZE,
    },
    resources::GameAssets,
};

use super::components::Player;

const PLAYER_SIZE: Vec2 = Vec2::new(48.0, 48.0);

pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>, windows: Query<&Window>) {
    let window = windows.single();

    commands
        .spawn(SpriteBundle {
            texture: game_assets.player.clone(),
            transform: Transform::from_xyz(0.0, (-window.height() + PLAYER_SIZE.x) / 2.0, 1.0),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            ..default()
        })
        .insert(Player {
            health: 20.0,
            speed: 150.0,
        });
}

pub fn movement(
    mut player_q: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
) {
    let (mut transform, player) = player_q.single_mut();

    if key_input.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }
    if key_input.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds();
    }
}

pub fn player_shooting(
    mut commands: Commands,
    player_q: Query<&GlobalTransform, With<Player>>,
    game_assets: Res<GameAssets>,
    mut mouse_event: EventReader<MouseButtonInput>,
) {
    for event in mouse_event.iter() {
        if let ButtonState::Pressed = event.state {
            let transform = player_q.single();
            let slipping: Vec3;

            if transform.translation().x as i32 % 2 == 0
                && transform.translation().x as i32 % 2 == 0
            {
                slipping = Vec3::new(14.0, 8.0, 0.0);
            } else {
                slipping = Vec3::new(-14.0, 8.0, 0.0);
            }

            commands
                .spawn(SpriteBundle {
                    texture: game_assets.bullet.clone(),
                    sprite: Sprite {
                        custom_size: Some(BULLET_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(transform.translation() + slipping),
                    ..default()
                })
                .insert(Bullet {
                    target_player: false,
                    speed: 100.0,
                    damage: 5.0,
                })
                .insert(LifeTime {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                });
        }
    }
}
