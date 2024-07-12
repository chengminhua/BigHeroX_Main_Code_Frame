use bevy::prelude::*;
use bigherox_robocup::{coach::CoachMode, MainPlugin, Mode};

fn main() {
    App::new()
        .add_plugins(MainPlugin {
            mode: Mode::Coach {
                mode: CoachMode::Normal,
            },
        })
        .run();
}
