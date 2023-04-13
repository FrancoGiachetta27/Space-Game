use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub full_background: Handle<Image>,
    pub player: Handle<Image>,
    pub bullet: Handle<Image>,
    pub enemy_1: Handle<Image>,
    //pub enemy_2: Handle<Image>,
    pub enemy_3: Handle<Image>,
}
