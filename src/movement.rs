use bevy::prelude::*;

use crate::{
    config::{OBSTACLE_SPEED, PLAYER_SPEED, WINDOW_HEIGHT, WINDOW_WIDTH},
    obstacles::Obstacle,
    state::GameState,
    Player,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Le mouvement ne tourne que pendant la partie : en game over, tout est figé.
        app.add_systems(
            Update,
            (character_movement, obstacle_movement).run_if(in_state(GameState::Playing)),
        );
    }
}

fn character_movement(
    mut characters: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // On construit un vecteur de direction à partir des touches...
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    // ... qu'on normalise (longueur 1) pour que la diagonale ne soit pas
    // plus rapide. `normalize_or_zero` évite la division par 0 à l'arrêt.
    let movement = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();

    for mut transform in &mut characters {
        transform.translation.x = (transform.translation.x + movement.x)
            .clamp(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.);
        transform.translation.y = (transform.translation.y + movement.y)
            .clamp(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.);
    }
}

fn obstacle_movement(mut obstacles: Query<(&mut Transform, &Obstacle)>, time: Res<Time>) {
    for (mut transform, obstacle) in &mut obstacles {
        transform.translation.x += obstacle.direction.x * OBSTACLE_SPEED * time.delta_seconds();
    }
}
