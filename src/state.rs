use bevy::prelude::*;

use crate::{config::PLAYER_START, events::PlayerHit, movement::Hop, obstacles::Obstacle, score::{update_high_score, HighScore, Score}, Player};

/// Les états du jeu. Bevy garde l'état courant dans une ressource et permet de
/// n'exécuter certains systèmes que dans un état donné (`run_if(in_state(..))`).
#[derive(States, Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

/// Marqueur : permet de retrouver et supprimer l'écran de game over.
#[derive(Component)]
struct GameOverUi;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            // `OnEnter` / `OnExit` : exécutés une seule fois, au changement d'état.
            .add_systems(
                OnEnter(GameState::GameOver),
                // Le record doit être à jour avant d'afficher l'écran de fin.
                show_game_over.after(update_high_score),
            )
            .add_systems(OnExit(GameState::GameOver), reset_game)
            .add_systems(
                Update,
                (
                    game_over_on_hit.run_if(in_state(GameState::Playing)),
                    restart_on_key.run_if(in_state(GameState::GameOver)),
                ),
            );
    }
}

fn show_game_over(mut commands: Commands, score: Res<Score>, high_score: Res<HighScore>) {
    // Un nœud plein écran qui centre son contenu, avec le texte en enfant.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GameOverUi,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!(
                        "Game Over\nScore : {}\nMeilleur : {}\nAppuie sur R pour rejouer",
                        score.0, high_score.0
                    ),
                    TextStyle {
                        font_size: 40.,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}

/// Réagit à la collision : passe en game over.
fn game_over_on_hit(
    mut hit_events: EventReader<PlayerHit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // `read().next()` : au moins un événement est arrivé.
    if hit_events.read().next().is_some() {
        next_state.set(GameState::GameOver);
    }
}

fn restart_on_key(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::KeyR) {
        // `set` ne change pas l'état tout de suite : la transition a lieu au
        // prochain passage du schedule `StateTransition`.
        next_state.set(GameState::Playing);
    }
}

/// Remet la partie à zéro quand on quitte `GameOver`.
fn reset_game(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player: Query<(Entity, &mut Transform), With<Player>>,
    obstacles: Query<Entity, With<Obstacle>>,
    ui: Query<Entity, With<GameOverUi>>,
) {
    score.0 = 0;
    for (entity, mut transform) in &mut player {
        transform.translation = PLAYER_START;
        // Annule un éventuel saut interrompu par le game over.
        commands.entity(entity).remove::<Hop>();
    }
    for entity in &obstacles {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }
}
