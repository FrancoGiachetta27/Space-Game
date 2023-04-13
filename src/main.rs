use bevy::{
    prelude::*,
    window::{WindowResized, WindowResolution},
};

mod bullet;
mod components;
mod enemy;
mod events;
mod player;
mod resources;
mod systems;

use systems::*;

use crate::{bullet::BulletPlugin, enemy::EnemyPlugin, player::PlayerPlugin};

const HEIGHT: f32 = 768.0;
const WIDTH: f32 = 1024.0;

#[derive(Component)]
struct BackGroundSprite;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system(asset_loader.in_base_set(StartupSet::PreStartup))
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(update_background_size.run_if(on_event::<WindowResized>()))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .run();
}
