use bevy::prelude::*;

use crate::{
    components::{Movable, Player, Position},
    map::MAP,
};

use super::Ghost;

#[derive(Component)]
pub struct Blinky;

pub fn blinky_ai(
    mut blinky_query: Query<(&mut Movable, &Position, &Ghost, &Blinky)>,
    pacman_query: Query<(&Position, &Player)>,
) {
    let (pacman_pos, _) = pacman_query
        .get_single()
        .expect("There should never be more than 1 player");

    for (mut blinky_movable, blinky_pos, _, _) in &mut blinky_query {
        let mut neighbours = MAP.get_empty_neighbours(&blinky_pos);

        neighbours.sort_by(|(a, _), (b, _)| {
            let dist_a = a.dist_to(pacman_pos);
            let dist_b = b.dist_to(pacman_pos);

            dist_b.partial_cmp(&dist_a).unwrap()
        });

        let (dest, dir) = neighbours.pop().expect("Blinky has nowhere to go");

        blinky_movable.direction = dir;
        blinky_movable.target_tile = dest;
    }
}
