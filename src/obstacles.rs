use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    assets::GameAssets,
    collision::Collider,
    config::{
        lane_y, LANES, LANE_SPAWN_RANGE, LANE_SPEED_RANGE, OBSTACLE_HALF_SIZE, SCALE, SCALED_TILE,
        TILE_SIZE, WINDOW_WIDTH,
    },
    score::Score,
    state::GameState,
};

/// Un obstacle en mouvement. `velocity` est signée : positive vers la droite,
/// négative vers la gauche (en pixels par seconde).
#[derive(Component)]
pub struct Obstacle {
    pub velocity: f32,
}

/// Une voie de circulation : elle fait apparaître des obstacles à intervalle
/// aléatoire, toujours dans le même sens et à la même vitesse.
#[derive(Component)]
pub struct Lane {
    /// +1. vers la droite, -1. vers la gauche.
    direction: f32,
    speed: f32,
    /// Chaque obstacle apparu tire un nouveau délai : le trafic est irrégulier.
    spawn_timer: Timer,
}

/// Abscisse au-delà de laquelle un obstacle est entièrement hors de l'écran.
const EDGE: f32 = WINDOW_WIDTH / 2. + SCALED_TILE;

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
        app.add_systems(Startup, setup_lanes).add_systems(
            Update,
            (spawn_obstacles, despawn_obstacles).run_if(in_state(GameState::Playing)),
        );
    }
}

/// Crée les voies : une entité par voie, avec une direction alternée.
fn setup_lanes(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for lane in 0..LANES {
        // Voies paires vers la droite, impaires vers la gauche.
        let direction = if lane % 2 == 0 { 1. } else { -1. };
        commands.spawn((
            Name::new(format!("Lane {lane}")),
            // Un `Transform` seul suffit à stocker la hauteur de la voie.
            Transform::from_xyz(0., lane_y(lane), 0.),
            Lane {
                direction,
                speed: rng.gen_range(LANE_SPEED_RANGE.0..LANE_SPEED_RANGE.1),
                spawn_timer: Timer::from_seconds(random_delay(&mut rng), TimerMode::Once),
            },
        ));
    }
}

fn random_delay(rng: &mut impl Rng) -> f32 {
    rng.gen_range(LANE_SPAWN_RANGE.0..LANE_SPAWN_RANGE.1)
}

fn spawn_obstacles(
    mut commands: Commands,
    mut lanes: Query<(&mut Lane, &Transform)>,
    time: Res<Time>,
    assets: Res<GameAssets>,
    score: Res<Score>,
) {
    // Le temps s'écoule plus vite pour les minuteurs quand la difficulté monte :
    // les obstacles apparaissent plus souvent, en phase avec leur vitesse.
    let scaled_delta = time.delta().mul_f32(score.speed_factor());
    let corners = square_corners(TILE_SIZE);
    let mut rng = rand::thread_rng();

    for (mut lane, lane_transform) in &mut lanes {
        lane.spawn_timer.tick(scaled_delta);
        if !lane.spawn_timer.finished() {
            continue;
        }

        // Prochain obstacle : nouveau délai aléatoire.
        let delay = Duration::from_secs_f32(random_delay(&mut rng));
        lane.spawn_timer.set_duration(delay);
        lane.spawn_timer.reset();

        // L'obstacle apparaît juste hors de l'écran, côté opposé à sa direction.
        let translation = Vec3::new(-lane.direction * EDGE, lane_transform.translation.y, 0.);

        // `with_children` permet de créer les 4 tuiles enfants en un seul bloc.
        // Leur transform est relatif au parent : déplacer le parent les déplace tous.
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(translation),
                    ..default()
                },
                Name::new("Obstacle"),
                Obstacle {
                    velocity: lane.direction * lane.speed,
                },
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

/// Supprime les obstacles sortis de l'écran *du côté où ils vont*
/// (sinon un obstacle tout juste apparu serait supprimé immédiatement).
fn despawn_obstacles(mut commands: Commands, obstacles: Query<(&Transform, &Obstacle, Entity)>) {
    for (transform, obstacle, entity) in &obstacles {
        if transform.translation.x * obstacle.velocity.signum() > EDGE {
            // `despawn_recursive` supprime aussi les tuiles enfants.
            commands.entity(entity).despawn_recursive();
        }
    }
}
