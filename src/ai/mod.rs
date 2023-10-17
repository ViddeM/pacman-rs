use bevy::prelude::*;

use crate::map::TilePos;

pub mod blinky;

#[derive(Component)]
pub struct Ghost {
    pub target_tile: TilePos,
    pub next_tile: TilePos,
}
