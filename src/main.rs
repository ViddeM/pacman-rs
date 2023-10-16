use ai::blinky::{blinky_ai, Blinky};
use ai::Ghost;
use bevy::{prelude::*, window::PresentMode};
use common::Direction;
use components::{AnimationIndices, AnimationTimer, Movable, Player, Position};
use map::MapType;
use map::{TilePos, MAP};
use player::{check_collision, move_player, steer};
use visuals::{animate_sprite, draw_movable, sprite_index_for_wall_type};

mod ai;
mod common;
mod components;
mod map;
mod math;
mod player;
mod visuals;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pacman-RS".into(),
                        resolution: (3000., 1600.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // set the global default background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_sprite,
                move_player,
                draw_movable,
                steer,
                check_collision,
                blinky_ai,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation = Vec3::new(70.0, -150.0, camera.transform.translation.z);
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
    let pacman_animation_indices = AnimationIndices::new(
        vec![24, 1, 0, 1],
        vec![24, 16, 15, 16],
        vec![24, 31, 30, 31],
        vec![24, 46, 45, 46],
    );
    let pacman_start_tile = TilePos { x: 13, y: 17 };
    commands.spawn((
        Position(pacman_start_tile.clone()),
        Movable::new(pacman_start_tile, 0.0, 11.5, Direction::Up),
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
    let blinky_animation_indices =
        AnimationIndices::new(vec![60, 61], vec![62, 63], vec![64, 65], vec![66, 67]);
    let blinky_start_tile = TilePos { x: 13, y: 11 };
    commands.spawn((
        Position(blinky_start_tile.clone()),
        Movable {
            base_speed: 7.0,
            direction: Direction::Up,
            target_tile: blinky_start_tile,
            progress: 0.0,
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(
                blinky_animation_indices.sprite_indices_right
                    [blinky_animation_indices.current_index],
            ),
            ..default()
        },
        Ghost,
        Blinky,
        blinky_animation_indices,
        AnimationTimer(Timer::from_seconds(1.0 / 8.0, TimerMode::Repeating)),
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

    // Spawn maze
    MAP.iter().enumerate().for_each(|(row_num, row)| {
        row.iter().enumerate().for_each(|(col_num, tile)| {
            let sprite_index = if let MapType::Wall(wall_type) = tile {
                sprite_index_for_wall_type(wall_type)
            } else {
                44
            };

            let tile_pos = TilePos {
                x: col_num as i32,
                y: row_num as i32,
            };
            let pos = tile_pos.to_display_pos();
            commands.spawn((
                Position(tile_pos),
                SpriteSheetBundle {
                    texture_atlas: texture_maze_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(sprite_index),
                    transform: Transform::from_translation(Vec3::new(pos.x, -pos.y, -1.0)),
                    ..default()
                },
            ));
        })
    })
}
