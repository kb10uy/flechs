//! Pong Tutorial 1

mod pong;
mod systems;

use crate::pong::{BallBundle, PaddleBundle, PaddleSide};

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    time::FixedTimestep,
};

fn main() {
    let fixed_update_system_set = SystemSet::new()
        .with_run_criteria(FixedTimestep::step(systems::FIXED_TIME_STEP as f64))
        .with_system(systems::move_paddle)
        .with_system(systems::move_ball)
        .with_system(systems::bounce_ball);

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .add_system_set(fixed_update_system_set)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("texture/pong_spritesheet.png");
    let texture_atlas = {
        let mut atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(8.0, 16.0));
        atlas.add_texture(Rect::new(0.0, 0.0, 4.0, 16.0));
        atlas.add_texture(Rect::new(4.0, 0.0, 8.0, 4.0));
        texture_atlasses.add(atlas)
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn(PaddleBundle::new(
        PaddleSide::Left,
        texture_atlas.clone(),
        0,
    ));
    commands.spawn(PaddleBundle::new(
        PaddleSide::Right,
        texture_atlas.clone(),
        0,
    ));
    commands.spawn(BallBundle::new(texture_atlas.clone(), 1));
}
