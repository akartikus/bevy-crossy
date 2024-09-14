use bevy::prelude::*;

use crate::{
    config::{GOAL_Y, PLAYER_START},
    movement::Hop,
    state::GameState,
    Player,
};

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        // `init_resource` crée la ressource avec sa valeur `Default`.
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_score_ui)
            .add_systems(
                Update,
                (
                    check_goal.run_if(in_state(GameState::Playing)),
                    update_score_text,
                ),
            );
    }
}

fn spawn_score_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Score : 0",
            TextStyle {
                font_size: 30.,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(8.),
            left: Val::Px(8.),
            ..default()
        }),
        ScoreText,
    ));
}

/// Quand le joueur atteint le haut : +1 point et retour au départ.
fn check_goal(
    mut score: ResMut<Score>,
    // `Without<Hop>` : on attend la fin du saut, sinon il écraserait la position remise au départ.
    mut player: Query<&mut Transform, (With<Player>, Without<Hop>)>,
) {
    for mut transform in &mut player {
        // Petite tolérance : l'interpolation du saut peut laisser un écart d'arrondi.
        if transform.translation.y >= GOAL_Y - 1. {
            score.0 += 1;
            transform.translation = PLAYER_START;
        }
    }
}

fn update_score_text(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    // `is_changed` est vrai seulement si la ressource a été modifiée depuis le
    // dernier passage de ce système : on évite de réécrire le texte à chaque frame.
    if !score.is_changed() {
        return;
    }
    for mut text in &mut text {
        text.sections[0].value = format!("Score : {}", score.0);
    }
}
