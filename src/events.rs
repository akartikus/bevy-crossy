use bevy::prelude::*;

/// Un événement est un message ponctuel : un système l'envoie (`EventWriter`),
/// d'autres le reçoivent (`EventReader`). L'émetteur ne sait pas qui l'écoute,
/// ce qui découple les modules (la collision ne connaît ni le score ni les états).

/// Le joueur a touché un obstacle.
#[derive(Event)]
pub struct PlayerHit;

/// Le joueur a atteint la ligne d'arrivée.
#[derive(Event)]
pub struct GoalReached;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        // Un événement doit être enregistré avant d'être utilisé.
        app.add_event::<PlayerHit>().add_event::<GoalReached>();
    }
}
