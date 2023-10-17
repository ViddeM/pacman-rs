use bevy::prelude::*;

use crate::ai::Ghost;
use crate::common::Direction;
use crate::components::{Movable, Player, Position};
use crate::map::{MapType, MAP};

pub fn check_collision(
    player_query: Query<(&Position, &Player)>,
    ghost_query: Query<(&Position, &Ghost)>,
) {
    for (player_pos, _) in player_query.iter() {
        for (ghost_pos, _) in ghost_query.iter() {
            if player_pos == ghost_pos {
                println!("Collision at tile {player_pos:?}");
            }
        }
    }
}

pub fn steer(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Movable, &Player>) {
    for mut movable in &mut query {
        let new_dir = if keyboard_input.just_pressed(KeyCode::W) {
            Direction::Up
        } else if keyboard_input.just_pressed(KeyCode::A) {
            Direction::Left
        } else if keyboard_input.just_pressed(KeyCode::S) {
            Direction::Down
        } else if keyboard_input.just_pressed(KeyCode::D) {
            Direction::Right
        } else {
            continue;
        };

        let new_target = movable.target_tile.translate(&new_dir);
        if !MAP.is_wall(&new_target) && MAP.get_at(&new_target) != MapType::GhostOnlyBarrier {
            movable.direction = new_dir;
        }
    }
}

pub fn move_player(time: Res<Time>, mut query: Query<(&mut Position, &mut Movable, &Player)>) {
    for (mut pos, mut movable, _) in &mut query {
        let delta = time.delta().as_secs_f32();
        let percent = movable.base_speed * delta;
        movable.progress = movable.progress + percent;

        if movable.progress >= 1.0 {
            movable.progress = 0.0;
            pos.0 = movable.target_tile.clone();

            let new_tile = pos.translate(&movable.direction);
            if !MAP.is_wall(&new_tile) {
                movable.target_tile = new_tile;
            }
        }
    }
}
