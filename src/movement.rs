use bevy::prelude::*;

use crate::{obstacles::Obstacle, Player, WINDOW_HEIGHT, WINDOW_WIDTH};
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
    let movement_amount = 150.0 * time.delta_seconds();
    for mut transform in &mut characters {
        if input.pressed(KeyCode::KeyW) && transform.translation.y < WINDOW_HEIGHT / 2. {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::KeyS) && transform.translation.y > -WINDOW_HEIGHT / 2. {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::KeyD) && transform.translation.x < WINDOW_WIDTH / 2. {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::KeyA) && transform.translation.x > -WINDOW_WIDTH / 2. {
            transform.translation.x -= movement_amount;
        }
    }
}
fn obstacle_movement(
    mut obstacles: Query<(&mut Transform, &Obstacle), With<Obstacle>>,
    time: Res<Time>,
) {
    for (mut transform, obstacle) in &mut obstacles {
        transform.translation.x += obstacle.direction.x * 150.0 * time.delta_seconds();
    }
}
