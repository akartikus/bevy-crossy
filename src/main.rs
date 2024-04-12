use bevy::{math::bounding::*, prelude::*, window::WindowResolution};
use movement::MovementPlugin;
use obstacles::ObstaclesPlugin;

mod movement;
mod obstacles;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteSheetBundle,
    player: Player,
}

#[derive(Component)]
enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component, Debug)]
enum CurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;
const SCALE: f32 = 2.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        title: String::from("Bevy crossy road"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MovementPlugin)
        .add_plugins(ObstaclesPlugin)
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
        27,
        18,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            texture: texture.clone(), // Use clone here to reuse the same texture handle
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(), // Clone the handle for reuse
                index: 24, // Assuming this index is for the player sprite
            },
            transform: {
                let mut transform = Transform::from_scale(Vec3::splat(SCALE));
                transform.translation.z = 10.0;
                transform.translation.y = 32.0 * SCALE;
                transform
            },
            ..default()
        },
        player: Player,
    });

    // fn draw_aabb_visualization(commands: &mut Commands, aabb: AABB) {
    //     let shape = shapes::Rectangle {
    //         extents: Vec2::new(aabb.max_x - aabb.min_x, aabb.max_y - aabb.min_y),
    //         origin: shapes::RectangleOrigin::Center,
    //     };

    //     commands.spawn(GeometryBuilder::build_as(
    //         &shape,
    //         DrawMode::Stroke(StrokeMode::new(Color::RED, 2.0)),
    //         Transform::from_xyz(
    //             (aabb.max_x + aabb.min_x) / 2.0,
    //             (aabb.max_y + aabb.min_y) / 2.0,
    //             0.0,
    //         ),
    //     ));
    // }
}
