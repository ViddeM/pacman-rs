use bevy::prelude::*;

use crate::common::Direction;
use crate::map::{Corner, WallType};
use crate::{
    components::{AnimationIndices, AnimationTimer, Movable, Position},
    math,
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &Movable,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut indices, movable, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let (new_index, new_sprite_index) = {
                let dir_indices = match movable.direction {
                    Direction::Up => &indices.sprite_indices_up,
                    Direction::Left => &indices.sprite_indices_left,
                    Direction::Right => &indices.sprite_indices_right,
                    Direction::Down => &indices.sprite_indices_down,
                };

                let index = (indices.current_index + 1) % dir_indices.len();
                (index, dir_indices[index])
            };

            indices.current_index = new_index;
            sprite.index = new_sprite_index;
        }
    }
}

pub fn get_display_pos(pos: &Position, movable: &Movable) -> Vec2 {
    let pos_a = pos.to_display_pos();
    let pos_b = movable.target_tile.to_display_pos();

    Vec2::new(
        math::lerp(pos_a.x, pos_b.x, movable.progress),
        math::lerp(pos_a.y, pos_b.y, movable.progress),
    )
}

pub fn draw_movable(mut query: Query<(&mut Transform, &Position, &Movable)>) {
    for (mut sprite, pos, movable) in &mut query {
        let p = get_display_pos(pos, movable);
        sprite.translation = Vec3::new(p.x, -p.y, sprite.translation.z);
    }
}

pub fn sprite_index_for_wall_type(wall_type: &WallType) -> usize {
    match wall_type {
        WallType::Straight(Direction::Up) => 20,
        WallType::Straight(Direction::Right) => 25,
        WallType::Straight(Direction::Down) => 14,
        WallType::Straight(Direction::Left) => 24,
        WallType::DoubleStraight(Direction::Up) => 10,
        WallType::DoubleStraight(Direction::Right) => 2,
        WallType::DoubleStraight(Direction::Down) => 12,
        WallType::DoubleStraight(Direction::Left) => 3,
        WallType::DoubleCorner(Corner::TopRight) => 0,
        WallType::DoubleCorner(Corner::BottomRight) => 4,
        WallType::DoubleCorner(Corner::BottomLeft) => 5,
        WallType::DoubleCorner(Corner::TopLeft) => 1,
        WallType::VerticalLineInnerCorner(Corner::TopRight) => 8,
        WallType::VerticalLineInnerCorner(Corner::BottomRight) => 6,
        WallType::VerticalLineInnerCorner(Corner::BottomLeft) => 7,
        WallType::VerticalLineInnerCorner(Corner::TopLeft) => 9,
        WallType::HorizontalLineInnerCornerTopRight => 43,
        WallType::HorizontalLineInnerCornerTopLeft => 42,
        WallType::OuterCorner(Corner::TopRight) => 27,
        WallType::OuterCorner(Corner::BottomRight) => 23,
        WallType::OuterCorner(Corner::BottomLeft) => 22,
        WallType::OuterCorner(Corner::TopLeft) => 26,
        WallType::InnerCorner(Corner::TopRight) => 35,
        WallType::InnerCorner(Corner::BottomRight) => 37,
        WallType::InnerCorner(Corner::BottomLeft) => 36,
        WallType::InnerCorner(Corner::TopLeft) => 34,
        WallType::NestCorner(Corner::TopRight) => 31,
        WallType::NestCorner(Corner::BottomRight) => 29,
        WallType::NestCorner(Corner::BottomLeft) => 28,
        WallType::NestCorner(Corner::TopLeft) => 30,
        WallType::Inner => 44,
    }
}
