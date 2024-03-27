use bevy::prelude::*;

struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("../assets/tilemap.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        26,
        18,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let sprite_index = 24;

    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: sprite_index,
        },
        transform: Transform::from_scale(Vec3::splat(5.0)),
        ..default()
    },));
}
