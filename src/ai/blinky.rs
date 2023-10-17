use bevy::prelude::*;

use crate::{
    common::Direction,
    components::{Movable, Player, Position},
    map::{MapType, TilePos, MAP},
};

use super::Ghost;

#[derive(Component)]
pub struct Blinky;

pub fn blinky_ai(
    time: Res<Time>,
    mut blinky_query: Query<(
        &mut Movable,
        &mut Position,
        &mut Ghost,
        &Blinky,
        Without<Player>,
    )>,
    pacman_query: Query<(&Position, &Player, Without<Ghost>)>,
) {
    let (pacman_pos, _, _) = pacman_query
        .get_single()
        .expect("There should never be more than 1 player");

    let delta = time.delta().as_secs_f32();
    for (mut blinky_movable, mut blinky_pos, mut ghost, _, _) in &mut blinky_query {
        handle_ai_decision(
            BlinkyDecision {
                pacman_pos: pacman_pos.clone(),
            },
            delta,
            &mut blinky_pos,
            &mut blinky_movable,
            &mut ghost,
        );
    }
}

trait AiDecision {
    fn take_decision(&self, position: &Position, movable: &Movable) -> TilePos;
}

fn handle_ai_decision(
    decision_maker: impl AiDecision,
    delta: f32,
    position: &mut Position,
    movable: &mut Movable,
    ghost: &mut Ghost,
) {
    let percent = movable.base_speed * delta;
    movable.progress = movable.progress + percent;
    let new_target_tile = decision_maker.take_decision(position, movable);
    ghost.target_tile = new_target_tile;

    if movable.progress >= 1.0 {
        movable.progress = 0.0;
        position.0 = movable.target_tile.clone();
        movable.target_tile = ghost.next_tile;

        let mut neighbours = MAP.get_empty_neighbours(&position);

        let next_tile = match neighbours.len() {
            0 => panic!("Ghost has nowhere to go :("),
            1 => neighbours.first().unwrap(),
            _ => {
                let mut choices = neighbours
                    .into_iter()
                    .filter(|(_, dir)| !movable.direction.opposite().eq(dir))
                    .filter(|(pos, dir)| match (MAP.get_at(pos), dir) {
                        (MapType::GhostOnlyBarrier, Direction::Down) => false,
                        _ => true,
                    })
                    .collect::<Vec<(TilePos, Direction)>>();

                neighbours.sort_by(|(a, _), (b, _)| {
                    let dist_a = a.dist_to(&ghost.target_tile);
                    let dist_b = b.dist_to(&ghost.target_tile);

                    dist_b.partial_cmp(&dist_a).unwrap()
                });

                neighbours.first().unwrap()
            }
        };
    }
}

struct BlinkyDecision {
    pacman_pos: Position,
}

impl AiDecision for BlinkyDecision {
    fn take_decision(&self, position: &Position, movable: &Movable) -> TilePos {
        movable.target_tile

        let mut neighbours = MAP
            .get_empty_neighbours(position)
            .into_iter()
            // Avoid doing 180s
            .filter(|(_, dir)| !movable.direction.opposite().eq(dir))
            // Avoid entering nest
            .filter(|(pos, _)| match MAP.get_at(pos) {
                MapType::GhostOnlyBarrier => false,
                _ => true,
            })
            .collect::<Vec<(TilePos, Direction)>>();

        neighbours.sort_by(|(a, _), (b, _)| {
            let dist_a = a.dist_to(&self.pacman_pos);
            let dist_b = b.dist_to(&self.pacman_pos);

            dist_b.partial_cmp(&dist_a).unwrap()
        });

        neighbours.pop().expect("Blinky has nowhere to go")
    }
}
