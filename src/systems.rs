use crate::{components::BackGroundSprite, resources::GameAssets, HEIGHT, WIDTH};
use bevy::{prelude::*, window::WindowResized};

pub fn setup(mut commands: Commands, game_assets: Res<GameAssets>, windows: Query<&Window>) {
    let window = windows.single();

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: game_assets.background.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            ..default()
        })
        .insert(BackGroundSprite);
}

pub fn update_background_size(
    mut commands: Commands,
    backgroud_q: Query<Entity, With<BackGroundSprite>>,
    game_assets: Res<GameAssets>,
    mut window_event: EventReader<WindowResized>,
) {
    let mut back_image: Handle<Image>;

    for background_ent in &backgroud_q {
        commands.entity(background_ent).despawn_recursive();

        for new_window in window_event.iter() {
            if new_window.width <= WIDTH && new_window.height <= HEIGHT {
                back_image = game_assets.background.clone();
            } else {
                back_image = game_assets.full_background.clone();
            }

            commands
                .spawn(SpriteBundle {
                    texture: back_image,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(new_window.width, new_window.height)),
                        ..default()
                    },
                    ..default()
                })
                .insert(BackGroundSprite);
        }
    }
}

pub fn asset_loader(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        background: asset_server.load("space_back.png"),
        full_background: asset_server.load("fullscreen_space_back.png"),
        player: asset_server.load("spaceship_p.png"),
        bullet: asset_server.load("bullet.png"),
        enemy_1: asset_server.load("enemy_1.png"),
        //enemy_2: asset_server.load("enemy_2.png"),
        enemy_3: asset_server.load("enemy_3.png"),
    });
}
