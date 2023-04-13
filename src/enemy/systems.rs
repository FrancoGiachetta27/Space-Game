use bevy::prelude::*;
use rand::Rng;

use crate::{
    bullet::{
        components::{Bullet, LifeTime},
        BULLET_SIZE,
    },
    player::components::Player,
    resources::GameAssets,
};

use super::{
    components::{Enemy, EnemyType},
    resources::EnemyConfig,
    ENEMY_SIZE,
};

pub fn spawn_enemy(
    mut commands: Commands,
    mut config: ResMut<EnemyConfig>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let spawn_range = (window.width() - ENEMY_SIZE.x) / 2.0;
    let abscissa_spawing_pos: f32 = rand::thread_rng().gen_range(-spawn_range..spawn_range); // substract half the enemy size to eliminate custom_size

    let enemy_type = match rand::thread_rng().gen_range(0..4) {
        0 => EnemyType::Normal,
        1 => EnemyType::DobleShooter,
        2 => EnemyType::CircleShooter,
        _ => EnemyType::AimShooter,
    };

    config.spawning_timer.tick(time.delta());

    if config.spawning_timer.just_finished() {
        let (assets, enemy) = enemy_type.get_enemy(&game_assets);

        commands
            .spawn(SpriteBundle {
                texture: assets,
                transform: Transform::from_xyz(
                    abscissa_spawing_pos,
                    (window.height() / 2.0) - (ENEMY_SIZE.y / 2.0),
                    1.0,
                ),
                sprite: Sprite {
                    flip_x: false,
                    flip_y: true,
                    custom_size: Some(ENEMY_SIZE),
                    ..default()
                },
                ..default()
            })
            .insert(enemy)
            .insert(enemy_type);
    }
}

pub fn avoid_bullets(
    mut enemy_q: Query<(&GlobalTransform, &mut Transform, &Enemy)>,
    bullet_q: Query<(&GlobalTransform, &Bullet)>,
    time: Res<Time>,
) {
    for (enemy_glob_transform, mut enemy_transform, enemy) in &mut enemy_q {
        for (bullet_glob_transform, bullet) in &bullet_q {
            let bullet_distance =
                bullet_glob_transform.translation() - enemy_glob_transform.translation();

            if bullet_distance.length() <= 40.0 && !bullet.target_player {
                enemy_transform.translation.x +=
                    -bullet_distance.x.signum() * enemy.speed * time.delta_seconds();
            }
        }
    }
}

pub fn avoid_other_enemies(
    mut enemy_q: Query<(Entity, &GlobalTransform, &mut Transform, &Enemy)>,
    mut colliders_q: Query<(Entity, &GlobalTransform), With<Enemy>>,
    time: Res<Time>,
) {
    let mut distance_collider: Vec3; // distance between the enemy and the possible collider

    for (enemy_entity, enemy_glob_transform, mut enemy_transform, enemy) in &mut enemy_q {
        for (col_entity, col_glob_transform) in &mut colliders_q {
            // exlude the same entity from the action
            if enemy_entity != col_entity {
                distance_collider =
                    col_glob_transform.translation() - enemy_glob_transform.translation();

                if distance_collider.length() <= 50.0 * (2.0 as f32).sqrt() {
                    if distance_collider.x.abs() <= 50.0 && distance_collider.y.abs() <= 50.0 {
                        enemy_transform.translation +=
                            -distance_collider.normalize() * enemy.speed * time.delta_seconds();
                    } else if distance_collider.x.abs() <= 50.0 && distance_collider.y.abs() >= 50.0
                    {
                        enemy_transform.translation.y +=
                            -distance_collider.y.signum() * enemy.speed * time.delta_seconds();
                    } else if distance_collider.y.abs() <= 50.0 && distance_collider.x.abs() >= 50.0
                    {
                        enemy_transform.translation.x +=
                            -distance_collider.x.signum() * enemy.speed * time.delta_seconds();
                    }
                }
            }
        }
    }
}

pub fn search_player(
    mut enemy_q: Query<(&GlobalTransform, &mut Transform, &Enemy)>,
    shot_q: Query<(&GlobalTransform, &Bullet)>,
    time: Res<Time>,
) {
    let mut shot_distance: Vec3;
    let last_shot = shot_q.iter().last();

    for (enemy_glob_transform, mut enemy_transform, enemy) in &mut enemy_q {
        if last_shot.is_some() {
            let (shot_position, bullet) = last_shot.unwrap();

            // we only need the last shot made get the player's x position
            if bullet.target_player {
                shot_distance = shot_position.translation() - enemy_glob_transform.translation();

                if shot_distance.x.abs() > 50.0 {
                    enemy_transform.translation.x +=
                        shot_distance.x.signum() * enemy.speed * time.delta_seconds();
                }
            } else {
                enemy_transform.translation.y -= enemy.speed * time.delta_seconds();
            }
        } else {
            enemy_transform.translation.y -= enemy.speed * time.delta_seconds();
        }
    }
}

pub fn enemy_shoot(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    enemy_q: Query<(&GlobalTransform, &EnemyType), With<Enemy>>,
    player_q: Query<&GlobalTransform, With<Player>>,
    mut config: ResMut<EnemyConfig>,
    time: Res<Time>,
) {
    let player_glob_transform = player_q.single();
    let mut player_distance: Vec3;
    // adds a little slipping so that the bullet sprite spawns in the right place
    let mut slipping: Vec3;
    config.shooting_timer.tick(time.delta());

    for (enemy_glob_transform, enemy_type) in &enemy_q {
        player_distance = enemy_glob_transform.translation() - player_glob_transform.translation();

        if player_distance.x.abs() <= 100.0 {
            if config.shooting_timer.just_finished() {
                let (asset, bullet) = enemy_type.get_bullet(&game_assets);

                if enemy_glob_transform.translation().x as i32 % 2 == 0
                    && enemy_glob_transform.translation().x as i32 % 2 == 0
                {
                    slipping = Vec3::new(14.0, 8.0, 0.0);
                } else {
                    slipping = Vec3::new(-14.0, 8.0, 0.0);
                }

                commands
                    .spawn(SpriteBundle {
                        texture: asset,
                        transform: Transform::from_translation(
                            enemy_glob_transform.translation() + slipping,
                        ),
                        sprite: Sprite {
                            flip_x: false,
                            flip_y: true,
                            custom_size: Some(BULLET_SIZE),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(bullet)
                    .insert(LifeTime {
                        timer: Timer::from_seconds(10.0, TimerMode::Once),
                    });
            }
        }
    }
}

pub fn enemy_death(mut commands: Commands, enemy_q: Query<(&Enemy, Entity)>) {
    for (enemy, enemy_ent) in &enemy_q {
        if enemy.health <= 0.0 {
            commands.entity(enemy_ent).despawn_recursive();
        }
    }
}
