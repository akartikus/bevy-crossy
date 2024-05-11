//! Constantes partagées par tout le jeu.

use bevy::math::{Vec2, Vec3};

pub const WINDOW_WIDTH: f32 = 700.;
pub const WINDOW_HEIGHT: f32 = 500.;

/// Facteur d'agrandissement des sprites (les tuiles font 16px, trop petit à l'écran).
pub const SCALE: f32 = 2.;
/// Taille d'une tuile de l'atlas, en pixels.
pub const TILE_SIZE: f32 = 16.;
/// Taille d'une tuile une fois agrandie.
pub const SCALED_TILE: f32 = TILE_SIZE * SCALE;

/// Position de départ du joueur : en bas de l'écran (z = 10 pour être devant les obstacles).
pub const PLAYER_START: Vec3 = Vec3::new(0., -WINDOW_HEIGHT / 2. + SCALED_TILE / 2., 10.);
/// Le joueur marque un point quand il atteint cette hauteur.
pub const GOAL_Y: f32 = WINDOW_HEIGHT / 2. - SCALED_TILE / 2.;

/// Demi-tailles des boîtes de collision. Celle du joueur est un peu plus petite
/// que son sprite (6 px au lieu de 8) pour que le jeu pardonne les frôlements.
pub const PLAYER_HALF_SIZE: Vec2 = Vec2::splat(6. * SCALE);
/// Un obstacle fait 2x2 tuiles : 32 px de côté avant agrandissement, donc 16 px de demi-côté.
pub const OBSTACLE_HALF_SIZE: Vec2 = Vec2::splat(16. * SCALE);

/// Vitesse du joueur et des obstacles, en pixels par seconde.
pub const PLAYER_SPEED: f32 = 150.;
pub const OBSTACLE_SPEED: f32 = 150.;
