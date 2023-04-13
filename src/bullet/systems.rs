use bevy::prelude::*;

use super::components::{Bullet, LifeTime};
use crate::{enemy::components::Enemy, player::components::Player};

pub fn bullet_movement(mut bullet_q: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullet_q {
        if bullet.target_player {
            transform.translation.y -= bullet.speed * time.delta_seconds();
        } else {
            transform.translation.y += bullet.speed * time.delta_seconds();
        }
    }
}

pub fn despawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut lifetime_q: Query<(Entity, &mut LifeTime)>,
) {
    for (ent, mut lifetime) in &mut lifetime_q {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.just_finished() {
            commands.entity(ent).despawn_recursive();
        }
    }
}

pub fn bullet_collition(
    mut commands: Commands,
    mut target_q: Query<(&Transform, &mut Enemy)>,
    bullet_q: Query<(&GlobalTransform, &Bullet, Entity)>,
    mut player_q: Query<(&Transform, &mut Player), With<Player>>,
) {
    let (player_transform, mut player) = player_q.single_mut();

    for (bullet_transform, bullet, bullet_ent) in &bullet_q {
        for (target_transform, mut enemy) in &mut target_q {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) <= 35.0
                && !bullet.target_player
            {
                enemy.health -= bullet.damage;

                commands.entity(bullet_ent).despawn_recursive();

                break;
            } else if Vec3::distance(bullet_transform.translation(), player_transform.translation)
                <= 35.0
                && bullet.target_player
            {
                player.health -= bullet.damage;

                commands.entity(bullet_ent).despawn_recursive();

                break;
            }
        }
    }
}
