use bevy::prelude::*;
use bigherox_robocup::{robot::RobotRole, MainPlugin, Mode};

fn main() {
    App::new()
        .add_plugins(MainPlugin {
            mode: Mode::Robot {
                role: RobotRole::Striker,
            },
        })
        .run();
}
