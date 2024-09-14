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

// --- Grille -----------------------------------------------------------------
// L'écran est découpé en lignes de la hauteur d'une tuile. La ligne 0 est en bas.

/// Nombre de lignes de la grille (500 / 32 = 15,6 : on en garde 15).
pub const ROWS: usize = 15;

/// Hauteur (centre) d'une ligne de la grille.
pub const fn row_y(row: usize) -> f32 {
    -WINDOW_HEIGHT / 2. + SCALED_TILE / 2. + row as f32 * SCALED_TILE
}

// --- Voies ------------------------------------------------------------------
// Un obstacle fait 2x2 tuiles : une voie occupe donc 2 lignes de la grille.
// Ligne 0 : départ, lignes 1 à 12 : 6 voies, lignes 13 et 14 : zone d'arrivée.

pub const LANES: usize = 6;

/// Hauteur (centre) de la voie `lane` : à cheval sur ses deux lignes.
pub const fn lane_y(lane: usize) -> f32 {
    row_y(1 + 2 * lane) + SCALED_TILE / 2.
}

/// Vitesse d'une voie, tirée au hasard dans cet intervalle (pixels par seconde).
pub const LANE_SPEED_RANGE: (f32, f32) = (80., 160.);
/// Délai entre deux obstacles d'une même voie, tiré au hasard (secondes).
pub const LANE_SPAWN_RANGE: (f32, f32) = (1.6, 3.2);

// --- Joueur -----------------------------------------------------------------

/// Position de départ du joueur : en bas de l'écran (z = 10 pour être devant les obstacles).
pub const PLAYER_START: Vec3 = Vec3::new(0., row_y(0), 10.);
/// Le joueur marque un point quand il atteint cette hauteur (dernière ligne).
pub const GOAL_Y: f32 = row_y(ROWS - 1);

/// Demi-tailles des boîtes de collision. Celle du joueur est un peu plus petite
/// que son sprite (6 px au lieu de 8) pour que le jeu pardonne les frôlements.
pub const PLAYER_HALF_SIZE: Vec2 = Vec2::splat(6. * SCALE);
/// Un obstacle fait 2x2 tuiles : 32 px de côté avant agrandissement, donc 16 px de demi-côté.
pub const OBSTACLE_HALF_SIZE: Vec2 = Vec2::splat(16. * SCALE);

/// Durée d'un saut d'une case, en secondes.
pub const HOP_DURATION: f32 = 0.12;
/// Nombre de colonnes de part et d'autre de la colonne centrale (21 colonnes au total).
pub const GRID_HALF_COLS: i32 = 10;
