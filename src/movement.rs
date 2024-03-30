use bevy::prelude::*;

use crate::{Obstacle, Player};
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (character_movement, obstacle_movement));
    }
}

fn character_movement(
    mut characters: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut characters {
        let movement_amount = 150.0 * time.delta_seconds();
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }
    }
}
fn obstacle_movement(mut obstacles: Query<&mut Transform, With<Obstacle>>, time: Res<Time>) {
    for mut transform in &mut obstacles {
        transform.translation.x += 150.0 * time.delta_seconds();
    }
}
