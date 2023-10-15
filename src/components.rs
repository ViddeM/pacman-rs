use bevy::prelude::*;

use crate::common::Direction;
use crate::map::TilePos;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut, PartialEq, Debug, Clone)]
pub struct Position(pub TilePos);

#[derive(Component, Clone, Debug)]
pub struct Movable {
    pub target_tile: TilePos,
    pub progress: f32,
    pub base_speed: f32, // Expressed in tiles per sec.
    pub direction: Direction,
}

impl Movable {
    pub fn new(target_tile: TilePos, progress: f32, base_speed: f32, direction: Direction) -> Self {
        Self {
            target_tile,
            progress,
            base_speed,
            direction,
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub current_index: usize,
    pub sprite_indices_right: Vec<usize>,
    pub sprite_indices_left: Vec<usize>,
    pub sprite_indices_up: Vec<usize>,
    pub sprite_indices_down: Vec<usize>,
}

impl AnimationIndices {
    pub fn new(right: Vec<usize>, left: Vec<usize>, up: Vec<usize>, down: Vec<usize>) -> Self {
        Self {
            current_index: 0,
            sprite_indices_right: right,
            sprite_indices_left: left,
            sprite_indices_up: up,
            sprite_indices_down: down,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
