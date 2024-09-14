use bevy::prelude::*;

use crate::{
    config::{row_y, GOAL_Y, GRID_HALF_COLS, HOP_DURATION, SCALED_TILE},
    obstacles::Obstacle,
    state::GameState,
    Player,
};

/// Un saut en cours : le joueur glisse de `from` à `to` en `HOP_DURATION` secondes.
///
/// C'est un composant *temporaire* : on l'ajoute au début du saut et on le retire
/// à la fin. Sa présence sert aussi de verrou : tant qu'il est là, on ignore
/// les nouvelles touches.
#[derive(Component)]
pub struct Hop {
    from: Vec3,
    to: Vec3,
    timer: Timer,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Le mouvement ne tourne que pendant la partie : en game over, tout est figé.
        // `chain` impose l'ordre : on lit les touches, puis on avance le saut.
        app.add_systems(
            Update,
            (
                (player_input, player_hop).chain(),
                obstacle_movement,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Démarre un saut d'une case si une touche est pressée et qu'aucun saut n'est en cours.
fn player_input(
    mut commands: Commands,
    // `Without<Hop>` : ne concerne que le joueur qui ne saute pas déjà.
    player: Query<(Entity, &Transform), (With<Player>, Without<Hop>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let direction = if input.pressed(KeyCode::KeyW) {
        Vec2::Y
    } else if input.pressed(KeyCode::KeyS) {
        Vec2::NEG_Y
    } else if input.pressed(KeyCode::KeyD) {
        Vec2::X
    } else if input.pressed(KeyCode::KeyA) {
        Vec2::NEG_X
    } else {
        return;
    };

    for (entity, transform) in &player {
        let from = transform.translation;
        let mut to = from + (direction * SCALED_TILE).extend(0.);
        // Reste dans la grille : on bloque le saut aux bords.
        let max_x = GRID_HALF_COLS as f32 * SCALED_TILE;
        to.x = to.x.clamp(-max_x, max_x);
        to.y = to.y.clamp(row_y(0), GOAL_Y);

        if to != from {
            commands.entity(entity).insert(Hop {
                from,
                to,
                timer: Timer::from_seconds(HOP_DURATION, TimerMode::Once),
            });
        }
    }
}

/// Fait avancer les sauts en cours et les termine.
fn player_hop(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut Hop)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut hop) in &mut player {
        hop.timer.tick(time.delta());
        // `fraction` va de 0 à 1 ; `lerp` interpole linéairement entre deux points.
        transform.translation = hop.from.lerp(hop.to, hop.timer.fraction());
        if hop.timer.finished() {
            commands.entity(entity).remove::<Hop>();
        }
    }
}

fn obstacle_movement(mut obstacles: Query<(&mut Transform, &Obstacle)>, time: Res<Time>) {
    for (mut transform, obstacle) in &mut obstacles {
        transform.translation.x += obstacle.velocity * time.delta_seconds();
    }
}
