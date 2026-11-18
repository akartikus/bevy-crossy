use bevy::prelude::*;

/// Un message est une information ponctuelle : un système l'envoie (`MessageWriter`),
/// d'autres le reçoivent (`MessageReader`). L'émetteur ne sait pas qui l'écoute,
/// ce qui découple les modules (la collision ne connaît ni le score ni les états).
/// (Dans les anciennes versions de Bevy, cela s'appelait `Event`.)

/// Le joueur a touché un obstacle.
#[derive(Message)]
pub struct PlayerHit;

/// Le joueur a atteint la ligne d'arrivée.
#[derive(Message)]
pub struct GoalReached;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        // Un message doit être enregistré avant d'être utilisé.
        app.add_message::<PlayerHit>().add_message::<GoalReached>();
    }
}
