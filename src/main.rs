use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
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
        27,
        18,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let sprite_index = 24;

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: sprite_index,
            },
            transform: Transform::from_scale(Vec3::splat(5.0)),
            ..default()
        })
        .insert(Player);

    let tile_obstacle_positions = vec![
        Vec3::new(-8.0, 8.0, 0.0),  // Top-Left
        Vec3::new(8.0, 8.0, 0.0),   // Top-Right
        Vec3::new(-8.0, -8.0, 0.0), // Bottom-Left
        Vec3::new(8.0, -8.0, 0.0),  // Bottom-Right
    ];

    let tile_obstacle_indices = vec![
        (27 * 16) + 15,
        (27 * 16) + 16,
        (27 * 17) + 15,
        (27 * 17) + 16,
    ];

    let parent_entity = commands
        .spawn((SpatialBundle::default(), Obstacle, Name::new("Obstacle")))
        .id();
    for (i, &tile_index) in tile_obstacle_indices.iter().enumerate() {
        // Fixme load once
        let texture = asset_server.load("../assets/tilemap.png");
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(16.0, 16.0),
            27,
            18,
            Some(Vec2::new(1.0, 1.0)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        commands.entity(parent_entity).with_children(|parent| {
            // parent.spawn_empty();
            parent.spawn(SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: tile_index,
                },
                transform: Transform {
                    translation: tile_obstacle_positions[i] * Vec3::splat(5.0),
                    scale: Vec3::splat(5.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
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
