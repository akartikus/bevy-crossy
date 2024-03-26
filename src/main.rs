use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, setup).run();
}

fn setup() {
    println!("Setup bevy!");
}