use bevy::{math::bounding::*, prelude::*};

use crate::{events::PlayerHit, obstacles::Obstacle, state::GameState, Player};

/// Boîte de collision, centrée sur l'entité. On stocke seulement la demi-taille :
/// la position vient du `Transform` de l'entité.
#[derive(Component)]
pub struct Collider {
    pub half_size: Vec2,
}

impl Collider {
    /// Construit la boîte AABB (Axis-Aligned Bounding Box : alignée sur les axes)
    /// à partir de la position monde de l'entité.
    fn aabb(&self, transform: &GlobalTransform) -> Aabb2d {
        Aabb2d::new(transform.translation().truncate(), self.half_size)
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_collisions.run_if(in_state(GameState::Playing)),
        );
    }
}

fn check_collisions(
    player: Query<(&GlobalTransform, &Collider), With<Player>>,
    obstacles: Query<(&GlobalTransform, &Collider), With<Obstacle>>,
    mut hit_events: EventWriter<PlayerHit>,
) {
    for (player_transform, player_collider) in &player {
        let player_box = player_collider.aabb(player_transform);
        for (obstacle_transform, obstacle_collider) in &obstacles {
            if player_box.intersects(&obstacle_collider.aabb(obstacle_transform)) {
                // On signale simplement la touche : c'est à d'autres systèmes de réagir.
                hit_events.send(PlayerHit);
                return;
            }
        }
    }
}
