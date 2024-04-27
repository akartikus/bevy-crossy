use assets::{GameAssets, GameAssetsPlugin};
use bevy::{prelude::*, window::WindowResolution};
use config::{SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use movement::MovementPlugin;
use obstacles::ObstaclesPlugin;

mod assets;
mod config;
mod movement;
mod obstacles;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteSheetBundle,
    player: Player,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        title: String::from("Bevy crossy road"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((GameAssetsPlugin, MovementPlugin, ObstaclesPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            texture: assets.texture.clone(),
            atlas: TextureAtlas {
                layout: assets.layout.clone(),
                index: 24, // index de la tuile du joueur dans l'atlas
            },
            transform: Transform {
                translation: Vec3::new(0., 32. * SCALE, 10.),
                scale: Vec3::splat(SCALE),
                ..default()
            },
            ..default()
        },
        player: Player,
    });
}
