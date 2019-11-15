use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::realm::Player;

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        let movement_x = input.axis_value("player_x").unwrap();
        let movement_y = input.axis_value("player_y").unwrap();
        for (_, transform) in (&players, &mut transforms).join() {
            transform.prepend_translation_x(movement_x as f32 * 5.0);
            transform.prepend_translation_y(movement_y as f32 * 5.0);
        }
    }
}
