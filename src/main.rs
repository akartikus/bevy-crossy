use bevy::{prelude::*, window::WindowResolution};
use movement::MovementPlugin;

mod movement;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;

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
                let mut transform = Transform::from_scale(Vec3::splat(4.0));
                transform.translation.z = 10.0;
                transform.translation.y = 32.0 * 5.0;
                transform
            },
            ..default()
        })
        .insert(Player);

    // Define positions for the tiles to create obstacles
    let tile_obstacle_positions = calculate_square_corners(16.0);

    // Indices for the tiles used as obstacles
    let tile_obstacle_indices = vec![
        (27 * 16) + 15,
        (27 * 16) + 16,
        (27 * 17) + 15,
        (27 * 17) + 16,
    ];

    // Spawn a parent entity for the obstacle tiles
    let parent_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(-WINDOW_WIDTH / 2., 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new("Obstacle"),
            Obstacle,
        ))
        .id();

    for (i, &tile_index) in tile_obstacle_indices.iter().enumerate() {
        commands.entity(parent_entity).with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture: texture.clone(), // Reuse the loaded texture
                atlas: TextureAtlas {
                    layout: texture_atlas_layout.clone(), // Reuse the atlas layout
                    index: tile_index,
                },
                transform: Transform {
                    translation: tile_obstacle_positions[i] * Vec3::splat(4.0),
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}

fn calculate_square_corners(center_distance: f32) -> Vec<Vec3> {
    let half_size = center_distance / 2.0; // Half the size of the square's side
    vec![
        Vec3::new(-half_size, half_size, 0.0),  // Top-Left corner
        Vec3::new(half_size, half_size, 0.0),   // Top-Right corner
        Vec3::new(-half_size, -half_size, 0.0), // Bottom-Left corner
        Vec3::new(half_size, -half_size, 0.0),  // Bottom-Right corner
    ]
}
