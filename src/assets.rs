use bevy::prelude::*;

/// Une `Resource` est une donnée globale unique, accessible dans n'importe quel
/// système via `Res<GameAssets>` (lecture) ou `ResMut<GameAssets>` (écriture).
///
/// Ici elle stocke les *handles* (des références légères et clonables) vers
/// la texture et le découpage de l'atlas. Les vraies données restent dans
/// `Assets<Image>` et `Assets<TextureAtlasLayout>`.
#[derive(Resource)]
pub struct GameAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        // `PreStartup` s'exécute entièrement avant `Startup`, et les commandes
        // (comme `insert_resource`) y sont appliquées avant la suite.
        // Ainsi, les systèmes `Startup` peuvent compter sur `Res<GameAssets>`.
        app.add_systems(PreStartup, load_assets);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Le chemin est relatif au dossier `assets/`.
    let texture = asset_server.load("tilemap.png");

    // Grille de 27 colonnes x 18 lignes de tuiles 16x16, séparées par 1px.
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(crate::config::TILE_SIZE as u32),
        27,
        18,
        Some(UVec2::splat(1)),
        None,
    ));

    commands.insert_resource(GameAssets { texture, layout });
}
