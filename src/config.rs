//! Constantes partagées par tout le jeu.

pub const WINDOW_WIDTH: f32 = 700.;
pub const WINDOW_HEIGHT: f32 = 500.;

/// Facteur d'agrandissement des sprites (les tuiles font 16px, trop petit à l'écran).
pub const SCALE: f32 = 2.;
/// Taille d'une tuile de l'atlas, en pixels.
pub const TILE_SIZE: f32 = 16.;
/// Taille d'une tuile une fois agrandie.
pub const SCALED_TILE: f32 = TILE_SIZE * SCALE;

/// Vitesse du joueur et des obstacles, en pixels par seconde.
pub const PLAYER_SPEED: f32 = 150.;
pub const OBSTACLE_SPEED: f32 = 150.;
