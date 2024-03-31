use bevy::{prelude::*, window::WindowResolution};
use movement::MovementPlugin;
use obstacles::ObstaclesPlugin;

mod movement;
mod obstacles;

#[derive(Component)]
struct Player;

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;
const SCALE: f32 = 2.;

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
        .add_plugins(MovementPlugin)
        .add_plugins(ObstaclesPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("../assets/tilemap.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        27,
        18,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteSheetBundle {
            texture: texture.clone(), // Use clone here to reuse the same texture handle
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(), // Clone the handle for reuse
                index: 24, // Assuming this index is for the player sprite
            },
            transform: {
                let mut transform = Transform::from_scale(Vec3::splat(SCALE));
                transform.translation.z = 10.0;
                transform.translation.y = 32.0 * 5.0;
                transform
            },
            ..default()
        })
        .insert(Player);
}
