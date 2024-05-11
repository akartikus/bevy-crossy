use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    assets::GameAssets,
    collision::Collider,
    config::{OBSTACLE_HALF_SIZE, SCALE, SCALED_TILE, TILE_SIZE, WINDOW_WIDTH},
    state::GameState,
};

#[derive(Component)]
pub struct Obstacle {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct ObstacleRoot {
    pub frequency: Timer,
}

/// Indices (dans l'atlas) des 4 tuiles qui forment un obstacle, dans le même
/// ordre que les coins renvoyés par `square_corners`.
const OBSTACLE_TILES: [usize; 4] = [
    27 * 16 + 15,
    27 * 16 + 16,
    27 * 17 + 15,
    27 * 17 + 16,
];

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_obstacle)
            .add_systems(
                Update,
                (spawn_obstacles, despawn_obstacles).run_if(in_state(GameState::Playing)),
            );
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
    assets: Res<GameAssets>,
) {
    let corners = square_corners(TILE_SIZE);

    for mut root in &mut obstacle_root {
        root.frequency.tick(time.delta());
        if !root.frequency.finished() {
            continue;
        }

        let direction = random_direction();
        let translation = random_translation(direction);

        // `with_children` permet de créer les 4 tuiles enfants en un seul bloc.
        // Leur transform est relatif au parent : déplacer le parent les déplace tous.
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(translation),
                    ..default()
                },
                Name::new("Obstacle"),
                Obstacle { direction },
                Collider {
                    half_size: OBSTACLE_HALF_SIZE,
                },
            ))
            .with_children(|parent| {
                for (corner, tile_index) in corners.iter().zip(OBSTACLE_TILES) {
                    parent.spawn(SpriteSheetBundle {
                        texture: assets.texture.clone(),
                        atlas: TextureAtlas {
                            layout: assets.layout.clone(),
                            index: tile_index,
                        },
                        transform: Transform {
                            translation: *corner * SCALE,
                            scale: Vec3::splat(SCALE),
                            ..default()
                        },
                        ..default()
                    });
                }
            });
    }
}

/// Positions des 4 tuiles autour du centre : haut-gauche, haut-droite,
/// bas-gauche, bas-droite.
fn square_corners(tile_size: f32) -> [Vec3; 4] {
    let half = tile_size / 2.;
    [
        Vec3::new(-half, half, 0.),
        Vec3::new(half, half, 0.),
        Vec3::new(-half, -half, 0.),
        Vec3::new(half, -half, 0.),
    ]
}

fn despawn_obstacles(mut commands: Commands, obstacles: Query<(&Transform, Entity), With<Obstacle>>) {
    let limit = WINDOW_WIDTH / 2. + SCALED_TILE;
    for (transform, entity) in &obstacles {
        if transform.translation.x.abs() > limit {
            // `despawn_recursive` supprime aussi les tuiles enfants.
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn random_translation(direction: Vec3) -> Vec3 {
    let mut y = 75.;
    let mut z = 0.;
    if direction.x < 0. {
        z = 1.;
        y += SCALED_TILE; // taille de l'obstacle + marge
    }
    let possible_positions = [
        Vec3::new(-WINDOW_WIDTH / 2., y, -z),
        Vec3::new(-WINDOW_WIDTH / 2., -y, z),
    ];
    let mut rng = rand::thread_rng();
    // Multiplier par `direction` inverse le x : un obstacle qui va vers la
    // gauche apparaît donc du côté droit.
    *possible_positions.choose(&mut rng).unwrap() * direction
}

fn random_direction() -> Vec3 {
    let possible_directions = [Vec3::new(-1., 1., 1.), Vec3::new(1., 1., 1.)];
    let mut rng = rand::thread_rng();
    *possible_directions.choose(&mut rng).unwrap()
}
