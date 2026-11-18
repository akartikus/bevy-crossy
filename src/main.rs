use assets::{GameAssets, GameAssetsPlugin};
use bevy::{prelude::*, window::WindowResolution};
use collision::{Collider, CollisionPlugin};
use events::EventsPlugin;
use config::{PLAYER_HALF_SIZE, PLAYER_START, SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use movement::MovementPlugin;
use obstacles::ObstaclesPlugin;
use score::ScorePlugin;
use state::GameStatePlugin;

mod assets;
mod collision;
mod config;
mod events;
mod movement;
mod obstacles;
mod score;
mod state;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: Sprite,
    transform: Transform,
    player: Player,
    collider: Collider,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                        title: String::from("Bevy crossy road"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            GameAssetsPlugin,
            EventsPlugin,
            GameStatePlugin,
            MovementPlugin,
            ObstaclesPlugin,
            CollisionPlugin,
            ScorePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2d);

    commands.spawn(PlayerBundle {
        // Un `Sprite` combine l'image et, ici, la tuile choisie dans l'atlas.
        sprite: Sprite::from_atlas_image(
            assets.texture.clone(),
            TextureAtlas {
                layout: assets.layout.clone(),
                index: 24, // index de la tuile du joueur dans l'atlas
            },
        ),
        transform: Transform {
            translation: PLAYER_START,
            scale: Vec3::splat(SCALE),
            ..default()
        },
        player: Player,
        collider: Collider {
            half_size: PLAYER_HALF_SIZE,
        },
    });
}
