use bevy::prelude::*;

use crate::{
    config::{GOAL_Y, PLAYER_START},
    movement::Hop,
    state::GameState,
    Player,
};

#[derive(Resource, Default)]
pub struct Score(pub u32);

/// Meilleur score de la session. Il survit aux parties (seul `Score` est remis à zéro).
#[derive(Resource, Default)]
pub struct HighScore(pub u32);

#[derive(Component)]
struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        // `init_resource` crée la ressource avec sa valeur `Default`.
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(Startup, spawn_score_ui)
            .add_systems(OnEnter(GameState::GameOver), update_high_score)
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
            "Score : 0   Meilleur : 0",
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

/// Au game over : retient le score s'il bat le record.
/// `pub` pour que l'écran de game over puisse s'exécuter *après* (`.after(...)`).
pub fn update_high_score(score: Res<Score>, mut high_score: ResMut<HighScore>) {
    high_score.0 = high_score.0.max(score.0);
}

fn update_score_text(
    score: Res<Score>,
    high_score: Res<HighScore>,
    mut text: Query<&mut Text, With<ScoreText>>,
) {
    // `is_changed` est vrai seulement si la ressource a été modifiée depuis le
    // dernier passage de ce système : on évite de réécrire le texte à chaque frame.
    if !score.is_changed() && !high_score.is_changed() {
        return;
    }
    for mut text in &mut text {
        text.sections[0].value = format!("Score : {}   Meilleur : {}", score.0, high_score.0);
    }
}
