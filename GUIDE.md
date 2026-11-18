# Guide du projet : Bevy Crossy

Projet éducatif pour maîtriser **Rust** et **Bevy 0.20** en faisant un clone de Crossy Road.
Ce fichier sert de feuille de route (ce qui est fait / à faire) et de support pour comprendre le code.

Lancer le jeu : `cargo run`. Contrôles : **WASD** pour sauter d'une case, **R** pour rejouer après un game over.

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
| `src/movement.rs` | Sauts du joueur (WASD) case par case et déplacement des obstacles |
| `src/obstacles.rs` | Voies de circulation, apparition et suppression des obstacles |
| `src/collision.rs` | `Collider` et détection de collision joueur / obstacle |
| `src/score.rs` | Score, meilleur score, difficulté, but à atteindre, affichage |
| `src/events.rs` | Événements `PlayerHit` et `GoalReached` |
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
  - `Node` est un conteneur avec des champs de style (comme du CSS flexbox). `Text`, `TextFont`, `TextColor` et `TextLayout` décrivent un texte.
  - Le composant marqueur `GameOverUi` permet de retrouver et supprimer l'écran de fin.
- [x] Le joueur démarre maintenant en bas de l'écran (avant, il était dans une voie d'obstacles).

### Étape 3 : design du jeu (fait)
- [x] **Système de voies** (`obstacles.rs`, `config.rs`)
  - L'écran est une grille de lignes de 32 px (`row_y`). Ligne 0 : départ. Lignes 13 et 14 : arrivée. Les 12 lignes du milieu forment 6 voies de 2 lignes (`lane_y`), car un obstacle fait 2x2 tuiles.
  - Chaque voie est une entité avec un composant `Lane` : direction (alternée), vitesse (aléatoire) et minuteur de spawn. Après chaque obstacle, un nouveau délai aléatoire est tiré : le trafic est irrégulier.
  - `Obstacle` ne contient plus qu'une `velocity` signée (+ vers la droite, - vers la gauche).
  - Les voies ne se chevauchent plus, et le hack sur `z` a disparu.
  - *Concepts : composant de configuration (`Lane`), entités sans sprite, `Timer` à durée variable.*
- [x] **Déplacement case par case** (`movement.rs`)
  - Une touche démarre un saut d'une case : on ajoute le composant `Hop { from, to, timer }`.
  - Pendant le saut, `player_hop` interpole la position (`Vec3::lerp` avec `timer.fraction()`), puis retire `Hop`.
  - `Query<..., Without<Hop>>` sert de verrou : pas de nouveau saut tant que le précédent n'est pas fini. Tenir la touche enchaîne les sauts.
  - `(player_input, player_hop).chain()` impose l'ordre d'exécution.
  - Le goal et le reset attendent la fin du saut (sinon `Hop` écraserait la position remise au départ).
  - *Concepts : composant temporaire comme machine à états, `Without<T>`, `chain`, interpolation.*
- [x] **Meilleur score** (`score.rs`)
  - `HighScore` est une `Resource` qui survit aux parties. Mise à jour dans `OnEnter(GameOver)` par `update_high_score`.
  - `show_game_over.after(update_high_score)` garantit que le record est à jour avant l'affichage.
  - Limite : le record n'est pas sauvegardé sur disque, il est perdu à la fermeture du jeu.

### Étape 4 : bonus
- [x] **Messages** (`events.rs`, appelés `Event` avant la migration vers Bevy 0.20)
  - `PlayerHit` (envoyé par `check_collisions`) et `GoalReached` (envoyé par `check_goal`).
  - `state.rs` écoute `PlayerHit` pour passer en game over, `score.rs` écoute `GoalReached` pour ajouter un point.
  - L'émetteur ne connaît pas les récepteurs : on peut en ajouter (sons, effets) sans toucher à la collision.
  - *Concepts : `#[derive(Message)]`, `add_message`, `MessageWriter::write`, `MessageReader::read`.*
- [x] **Difficulté progressive**
  - `Score::speed_factor()` vaut 1.0 au départ, +0.1 par point, plafonné à 2.5 (`config.rs`).
  - Les obstacles vont plus vite, et les minuteurs de spawn avancent plus vite aussi (`delta.mul_f32(facteur)`), donc l'espacement entre obstacles reste constant.
- [ ] **Animation du sprite** : non fait. Aucune frame d'animation du joueur n'est clairement identifiable dans la tilemap, et je ne peux pas lancer le jeu pour vérifier. Pour le faire : choisir les indices des frames, puis un composant `AnimationTimer` qui change `TextureAtlas.index`.
- [ ] **Sons** : non fait. Il n'y a aucun fichier audio dans `assets/`. Pour le faire : ajouter des `.ogg` dans `assets/`, puis jouer un `AudioPlayer` en réaction à `PlayerHit` et `GoalReached`.
- [ ] **Caméra qui suit le joueur** : non fait. Le jeu tient sur un seul écran fixe : il faudrait d'abord un monde plus grand (génération de voies à l'infini), ce qui change le design.
- [x] **Migration vers Bevy 0.20** (et `rand` 0.10) : voir la section suivante.


---

## Migration vers Bevy 0.20

Le projet est passé de Bevy 0.13 à 0.20 (et `rand` de 0.8 à 0.10). Les changements qui ont touché le code :

| Avant (0.13) | Maintenant (0.20) |
|---|---|
| `Camera2dBundle::default()` | `Camera2d` (un composant, qui en ajoute d'autres tout seul) |
| `SpriteSheetBundle { texture, atlas, transform }` | `Sprite::from_atlas_image(image, TextureAtlas { .. })` + `Transform` |
| `SpatialBundle` | `Transform` + `Visibility` |
| `NodeBundle { style: Style { .. } }` | `Node { .. }` |
| `TextBundle::from_section(..)` | `Text::new(..)` + `TextFont` + `TextColor` |
| `text.sections[0].value` | `text.0` |
| `Event`, `EventWriter::send`, `EventReader` | `Message`, `MessageWriter::write`, `MessageReader` |
| `despawn_recursive()` | `despawn()` (supprime aussi les enfants) |
| `time.delta_seconds()` | `time.delta_secs()` |
| `timer.finished()` | `timer.is_finished()` |
| `bevy::math::bounding` | `bevy::shape` (`Aabb2d`, `IntersectsVolume`) |
| `TextureAtlasLayout::from_grid(Vec2, ..)` | tailles en `UVec2` |
| `WindowResolution::new(f32, f32)` | `WindowResolution::new(u32, u32)` |
| `rand::thread_rng()`, `gen_range` | `rand::rng()`, `random_range` |

Le gros changement d'idée : les bundles ont presque disparu au profit des **composants requis** (*required components*). Un `Sprite` demande automatiquement un `Transform` et une `Visibility`, un `Text` demande ses composants de mise en page, etc. On spawn donc un tuple de composants.

Cette migration a été faite en suivant les erreurs du compilateur et le code source des crates, mais elle n'a pas été testée en lançant le jeu.

---

## Pistes d'amélioration repérées (hors étapes)

- Sauvegarder le meilleur score sur disque (fichier JSON dans le dossier de l'utilisateur).
- Les index de tuiles (`27 * 16 + 15`...) sont en dur. Un petit module de constantes d'atlas serait plus lisible.
- Les tailles des boîtes de collision (`config.rs`) sont des estimations à ajuster en jouant.
- Un écran de titre (nouvel état `Menu`) serait un bon exercice sur les états.
