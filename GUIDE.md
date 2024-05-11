# Guide du projet : Bevy Crossy

Projet éducatif pour maîtriser **Rust** et **Bevy 0.13** en faisant un clone de Crossy Road.
Ce fichier sert de feuille de route (ce qui est fait / à faire) et de support pour comprendre le code.

Lancer le jeu : `cargo run`. Contrôles : **WASD** pour bouger, **R** pour rejouer après un game over.

---

## Concepts Bevy de base (ECS)

Bevy utilise l'architecture **ECS** (Entity Component System) :

| Terme | Rôle | Dans ce projet |
|---|---|---|
| **Entity** | Un simple identifiant, sans données | Le joueur, un obstacle, un texte |
| **Component** | Une donnée attachée à une entité | `Player`, `Obstacle`, `Collider`, `Transform` |
| **System** | Une fonction qui agit sur les entités | `character_movement`, `check_collisions` |
| **Resource** | Une donnée globale unique | `GameAssets`, `Score` |
| **Plugin** | Un groupe de systèmes/ressources réutilisable | `MovementPlugin`, `CollisionPlugin` |
| **State** | Une phase du jeu qui active/désactive des systèmes | `GameState::Playing` / `GameOver` |

Un système déclare ce dont il a besoin dans ses paramètres, et Bevy les fournit :
- `Query<&mut Transform, With<Player>>` : les entités qui ont un `Transform` et un `Player`.
- `Res<T>` / `ResMut<T>` : lire / modifier une ressource.
- `Commands` : créer ou supprimer des entités (appliqué un peu plus tard, pas instantanément).

---

## Organisation du code

| Fichier | Contenu |
|---|---|
| `src/main.rs` | Point d'entrée : fenêtre, plugins, spawn de la caméra et du joueur |
| `src/config.rs` | Constantes partagées (tailles, vitesses, positions) |
| `src/assets.rs` | `GameAssets` : texture et atlas chargés une seule fois |
| `src/movement.rs` | Déplacement du joueur (WASD) et des obstacles |
| `src/obstacles.rs` | Apparition et suppression des obstacles |
| `src/collision.rs` | `Collider` et détection de collision joueur / obstacle |
| `src/score.rs` | Score, but à atteindre, affichage du score |
| `src/state.rs` | `GameState`, écran de game over, redémarrage |

---

## Feuille de route

### Étape 0 : déjà présent au départ
- [x] Fenêtre, caméra 2D, sprite du joueur depuis un atlas (`tilemap.png`)
- [x] Déplacement WASD
- [x] Apparition d'obstacles toutes les secondes, dans un sens aléatoire, puis suppression hors écran

### Étape 1 : bases propres (fait)
- [x] **`GameAssets` (Resource)** : la texture et le layout de l'atlas sont créés une seule fois dans `PreStartup`.
  Avant, le layout était recréé à chaque frame, ce qui était une fuite mémoire.
  *Concepts : `Resource`, `Handle`, `PreStartup`.*
- [x] **`config.rs`** : fini les nombres magiques (`16.0 * SCALE`, `150.`) répétés partout.
- [x] **Mouvement normalisé** : en diagonale, le joueur n'est plus plus rapide (`normalize_or_zero`), et `clamp` remplace les `if` de bordure.
  *Concepts : `Vec2`, normalisation de vecteur.*
- [x] Nettoyage du code mort et des idiomes Rust (`zip`, `choose`, tableaux `[T; N]`).

### Étape 2 : le jeu devient jouable (fait)
- [x] **Collisions** (`collision.rs`)
  - Le composant `Collider` stocke une demi-taille. La position vient du `GlobalTransform`.
  - `Aabb2d::new(centre, demi_taille)` construit une boîte, `.intersects(&autre)` teste le chevauchement.
  - La boîte du joueur est volontairement un peu plus petite que son sprite.
  - *Concepts : composant, AABB, `GlobalTransform` (position réelle dans le monde, même pour une entité enfant).*
- [x] **États du jeu** (`state.rs`)
  - `GameState` (`Playing` / `GameOver`) dérive `States`. `init_state` l'enregistre.
  - `run_if(in_state(GameState::Playing))` : mouvement, apparition et collisions ne tournent que pendant la partie. En game over, tout est figé.
  - `OnEnter(GameOver)` affiche l'écran de fin, `OnExit(GameOver)` remet tout à zéro.
  - `next_state.set(...)` demande un changement d'état, appliqué au prochain `StateTransition`.
  - *Concepts : `States`, `NextState`, `OnEnter`, `OnExit`, run conditions.*
- [x] **But et score** (`score.rs`)
  - Atteindre le haut de l'écran (`GOAL_Y`) donne +1 point et ramène le joueur en bas.
  - `Score` est une `Resource`. Le texte n'est mis à jour que si `score.is_changed()`.
  - *Concepts : `init_resource`, `Default`, détection de changement.*
- [x] **Interface (bevy_ui)** : texte du score en haut à gauche, écran de game over centré.
  - `NodeBundle` est un conteneur avec `Style` (comme du CSS flexbox). `TextBundle` affiche du texte.
  - Le composant marqueur `GameOverUi` permet de retrouver et supprimer l'écran de fin.
- [x] Le joueur démarre maintenant en bas de l'écran (avant, il était dans une voie d'obstacles).

### Étape 3 : design du jeu (à faire)
- [ ] **Système de voies (`Lane`)** : chaque voie a sa position `y`, sa direction, sa vitesse et sa fréquence d'apparition. Cela remplace le `y = 75` fixe et la logique confuse de `randomize_translation`.
- [ ] **Déplacement case par case** : un appui = un saut d'une case, avec une interpolation fluide (le vrai gameplay de Crossy Road).
- [ ] **Meilleur score** (`HighScore`) affiché à côté du score.

### Étape 4 : bonus (à faire)
- [ ] Événements Bevy (`Event`, `EventWriter`, `EventReader`) pour la collision et le score
- [ ] Animation du sprite avec l'atlas
- [ ] Sons (`AudioBundle`)
- [ ] Difficulté progressive (vitesse et fréquence qui augmentent avec le score)
- [ ] Caméra qui suit le joueur
- [ ] Migration vers une version récente de Bevy (l'API a beaucoup changé depuis la 0.13)

---

## Pistes d'amélioration repérées (hors étapes)

- Les voies actuelles se chevauchent en `y` (obstacles à 75 et 107 : 64 px de haut chacun). Le système de voies de l'étape 3 règle ce défaut.
- L'ordre d'exécution des systèmes dans `Update` n'est pas garanti. Par exemple, la collision peut être testée avant ou après le mouvement de la même frame. Plus tard, on pourra utiliser `.chain()` ou `.after(...)`.
- Les index de tuiles (`27 * 16 + 15`...) sont en dur. Un petit module de constantes d'atlas serait plus lisible.
