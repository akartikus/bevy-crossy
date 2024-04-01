use bevy::prelude::*;
use rand::prelude::*;

use crate::{SCALE, WINDOW_WIDTH};

#[derive(Component)]
pub struct Obstacle {
    pub direction: Vec3,
}
#[derive(Component)]
pub struct ObstacleRoot {
    pub frequency: Timer,
}

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_obstacle)
            .add_systems(Update, (spawn_obstacles, despawn_obstacles));
    }
}

fn setup_obstacle(mut commands: Commands) {
    commands.spawn(ObstacleRoot {
        frequency: Timer::from_seconds(1., TimerMode::Repeating),
    });
}

fn spawn_obstacles(
    mut commands: Commands,
    mut obstacle_root: Query<&mut ObstacleRoot>,
    time: Res<Time>,
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

    // Define positions for the tiles to create obstacles
    let tile_obstacle_positions = calculate_square_corners(16.0);

    // Indices for the tiles used as obstacles
    let tile_obstacle_indices = vec![
        (27 * 16) + 15,
        (27 * 16) + 16,
        (27 * 17) + 15,
        (27 * 17) + 16,
    ];

    for mut obstacle in &mut obstacle_root {
        obstacle.frequency.tick(time.delta());
        if obstacle.frequency.finished() {
            // let tranmslation = rando
            // Spawn root for the obstacle tiles
            let direction = randomize_direction();
            let translation = randomize_translation(direction);
            let obsctacle_entity = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform {
                            translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Name::new("Obstacle"),
                    Obstacle { direction },
                ))
                .id();

            for (i, &tile_index) in tile_obstacle_indices.iter().enumerate() {
                commands.entity(obsctacle_entity).with_children(|parent| {
                    parent.spawn(SpriteSheetBundle {
                        texture: texture.clone(), // Reuse the loaded texture
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout.clone(), // Reuse the atlas layout
                            index: tile_index,
                        },
                        transform: Transform {
                            translation: tile_obstacle_positions[i] * Vec3::splat(SCALE),
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            }
        }
    }
}
fn calculate_square_corners(center_distance: f32) -> Vec<Vec3> {
    let half_size = center_distance / 2.0; // Half the size of the square's side
    vec![
        Vec3::new(-half_size, half_size, 0.0),  // Top-Left corner
        Vec3::new(half_size, half_size, 0.0),   // To>p-Right corner
        Vec3::new(-half_size, -half_size, 0.0), // Bottom-Left corner
        Vec3::new(half_size, -half_size, 0.0),  // Bottom-Right corner
    ]
}
fn despawn_obstacles(
    mut commands: Commands,
    mut obstacles: Query<(&Transform, Entity), With<Obstacle>>,
) {
    let right_boundary = WINDOW_WIDTH / 2. + 16.0 * SCALE;
    let left_boundary = -WINDOW_WIDTH / 2. - 16.0 * SCALE;
    for (transform, entity) in &mut obstacles {
        let x_position = transform.translation.x;
        if x_position > right_boundary || x_position < left_boundary {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn randomize_translation(direction: Vec3) -> Vec3 {
    let mut y = 75.;
    let mut z = 0.;
    if direction.x < 0. {
        z = 1.;
        y = y + 16. * SCALE; // obstacle size + margin
    }
    let possible_positions = vec![
        Vec3::new(-WINDOW_WIDTH / 2., y, -z),
        Vec3::new(-WINDOW_WIDTH / 2., -y, z),
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible_positions.len());
    return possible_positions[index] * direction;
}

fn randomize_direction() -> Vec3 {
    let possible_directions = vec![Vec3::new(-1., 1., 1.), Vec3::new(1., 1., 1.)];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..possible_directions.len());
    return possible_directions[index];
}
