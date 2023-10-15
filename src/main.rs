use bevy::prelude::*;
use common::Direction;
use map::{Corner, WallType, MAP};

use crate::map::MapType;

mod common;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // set the global default background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, move_character)
        .add_systems(Update, move_sprite)
        .add_systems(Update, steer)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, DerefMut, Deref)]
struct Position(Vec2);

#[derive(Component)]
struct Movable {
    base_speed: f32,
    direction: Direction,
}

#[derive(Component)]
struct AnimationIndices {
    current_index: usize,
    sprite_indices_right: Vec<usize>,
    sprite_indices_left: Vec<usize>,
    sprite_indices_up: Vec<usize>,
    sprite_indices_down: Vec<usize>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
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

fn move_character(time: Res<Time>, mut query: Query<(&mut Position, &Movable)>) {
    for (mut pos, movable) in &mut query {
        let amount_to_move = movable.base_speed * time.delta().as_secs_f32();
        let new_pos = match movable.direction {
            Direction::Up => Vec2::new(pos.x, pos.y + amount_to_move),
            Direction::Right => Vec2::new(pos.x + amount_to_move, pos.y),
            Direction::Down => Vec2::new(pos.x, pos.y - amount_to_move),
            Direction::Left => Vec2::new(pos.x - amount_to_move, pos.y),
        };
        pos.0.x = new_pos.x;
        pos.0.y = new_pos.y;
    }
}

fn move_sprite(mut query: Query<(&mut Transform, &Position)>) {
    for (mut sprite, pos) in &mut query {
        sprite.translation = Vec3::new(pos.x, pos.y, sprite.translation.z);
    }
}

fn steer(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Movable, &Player>) {
    for mut movable in &mut query {
        if keyboard_input.just_pressed(KeyCode::W) {
            movable.direction = Direction::Up;
        }
        if keyboard_input.just_pressed(KeyCode::A) {
            movable.direction = Direction::Left;
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            movable.direction = Direction::Down;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            movable.direction = Direction::Right;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    commands.spawn(camera);

    spawn_characters(&mut commands, &asset_server, &mut texture_atlases);
    spawn_map(&mut commands, &asset_server, &mut texture_atlases);
}

fn spawn_characters(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let sprite_handle = asset_server.load("sprites/pacman_character_sheet_2.png");
    let atlas = TextureAtlas::from_grid(sprite_handle, Vec2::new(16.0, 16.0), 15, 15, None, None);
    let texture_atlas_handle = texture_atlases.add(atlas);

    // Pacman
    let pacman_animation_indices = AnimationIndices {
        current_index: 0,
        sprite_indices_right: vec![24, 1, 0, 1],
        sprite_indices_down: vec![24, 46, 45, 46],
        sprite_indices_left: vec![24, 16, 15, 16],
        sprite_indices_up: vec![24, 31, 30, 31],
    };
    commands.spawn((
        Position(Vec2::new(0.0, 0.0)),
        Movable {
            base_speed: 80.0,
            direction: Direction::Right,
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(
                pacman_animation_indices.sprite_indices_right
                    [pacman_animation_indices.current_index],
            ),
            ..default()
        },
        pacman_animation_indices,
        Player,
        AnimationTimer(Timer::from_seconds(1.0 / 16.0, TimerMode::Repeating)),
    ));

    // Blinky
    let blinky_animation_indices = AnimationIndices {
        current_index: 0,
        sprite_indices_right: vec![60, 61],
        sprite_indices_left: vec![62, 63],
        sprite_indices_up: vec![64, 65],
        sprite_indices_down: vec![66, 67],
    };
    commands.spawn((
        Position(Vec2::new(0.0, 0.0)),
        Movable {
            base_speed: 80.0,
            direction: Direction::Up,
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(
                blinky_animation_indices.sprite_indices_right
                    [blinky_animation_indices.current_index],
            ),
            ..default()
        },
        blinky_animation_indices,
        AnimationTimer(Timer::from_seconds(1.0 / 16.0, TimerMode::Repeating)),
    ));
}

fn spawn_map(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let maze_sprite_handle = asset_server.load("sprites/pacman_maze.png");
    let maze_atlas = TextureAtlas::from_grid(
        maze_sprite_handle,
        Vec2::new(8.0, 8.0),
        16,
        14,
        Some(Vec2::new(1.0, 1.0)),
        Some(Vec2::new(225.0, 27.0)),
    );
    let texture_maze_atlas_handle = texture_atlases.add(maze_atlas);

    let map_x_offset = -100.0;
    let map_y_offset = 100.0;

    // Spawn maze
    MAP.iter().enumerate().for_each(|(row_num, row)| {
        row.iter().enumerate().for_each(|(col_num, tile)| {
            let sprite_index = if let MapType::Wall(wall_type) = tile {
                sprite_index_for_wall_type(wall_type)
            } else {
                44
            };

            let x = (col_num as f32) * 8.0 + map_x_offset;
            let y = -(row_num as f32) * 8.0 + map_y_offset;
            commands.spawn((
                Position(Vec2::new(x, y)),
                SpriteSheetBundle {
                    texture_atlas: texture_maze_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(sprite_index),
                    transform: Transform::from_translation(Vec3::new(x, y, -1.0)),
                    ..default()
                },
            ));
        })
    })
}

fn sprite_index_for_wall_type(wall_type: &WallType) -> usize {
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
