# bevy-crossy

Un petit clone de Crossy Road fait avec Rust et Bevy 0.13. Je l'ai commencé pour apprendre Rust et Bevy, donc le code est plein de commentaires qui expliquent ce que je découvre en route.

Le but : traverser la route sans se faire toucher. Chaque traversée rapporte un point, et plus le score monte, plus le trafic va vite.

## Lancer le jeu

Il faut Rust installé (rustup), puis :

    cargo run

La première compilation est longue à cause de Bevy, les suivantes vont beaucoup plus vite.

## Contrôles

WASD pour sauter d'une case. Si tu restes appuyé, ça enchaîne les sauts. Après un game over, R relance une partie.

## Comment c'est rangé

Tout est dans `src/`, un fichier par sujet :

- `main.rs` : la fenêtre, la caméra et le joueur
- `config.rs` : les constantes (tailles, grille, vitesses...)
- `assets.rs` : le chargement de la tilemap, une seule fois
- `movement.rs` : les sauts du joueur et le déplacement des obstacles
- `obstacles.rs` : les voies et l'apparition des obstacles
- `collision.rs` : la détection des collisions
- `score.rs` : le score, le meilleur score et la difficulté
- `state.rs` : les états du jeu et l'écran de game over
- `events.rs` : les événements échangés entre les systèmes

Le fichier `GUIDE.md` détaille ce qui a été fait, les notions de Bevy utilisées à chaque étape et ce qu'il reste à faire.

## Ce qui manque

Pas de sons, pas d'animation du joueur, et le meilleur score n'est pas sauvegardé quand on ferme le jeu. Les réglages de collision et de vitesse sont à ajuster à la main dans `config.rs` si le jeu paraît trop dur ou trop facile.

## Crédits

Les sprites viennent de [Kenney](https://kenney.nl) (tilemap 16x16 dans `assets/tilemap.png`). Ses assets sont libres de droits (CC0), merci à lui.
