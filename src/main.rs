mod chart;
mod easing;
mod game;
mod rendering;
mod timing;

use bevy::prelude::*;
use bevy::window::WindowMode;
use std::fs;

use chart::Chart;
use game::{initialize_canvas_states, GameState};
use rendering::{Background, RenderingPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CH-RZL Player".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: true,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RenderingPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    // Load chart from file or use default
    let chart_path = std::env::args().nth(1).unwrap_or_else(|| "morimoriatsushi0IN.json".to_string());
    
    let chart = if let Ok(json) = fs::read_to_string(&chart_path) {
        match Chart::from_json(&json) {
            Ok(c) => {
                println!("Loaded chart: {}", chart_path);
                c
            }
            Err(e) => {
                eprintln!("Failed to parse chart {}: {}", chart_path, e);
                create_empty_chart()
            }
        }
    } else {
        eprintln!("Chart file not found: {}", chart_path);
        create_empty_chart()
    };

    let mut game_state = GameState::new(chart);
    initialize_canvas_states(&mut game_state);

    commands.insert_resource(game_state);

    // Setup 2D camera with offset for game coordinate system
    // Offset calculation: 200 * (WINDOW_HEIGHT / 640.0) = 284.375
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 284.375, 0.0),
        ..default()
    });

    // Spawn background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Background,
    ));

    println!("CH-RZL Player initialized!");
    println!("Controls:");
    println!("  Space - Play/Pause");
    println!("  R - Reset to beginning");
    println!("  Left/Right - Seek backward/forward");
    println!("  Up/Down - Adjust speed");
}

fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    // Toggle play/pause
    if keyboard.just_pressed(KeyCode::Space) {
        game_state.is_playing = !game_state.is_playing;
        println!("Playing: {}", game_state.is_playing);
    }

    // Reset
    if keyboard.just_pressed(KeyCode::KeyR) {
        game_state.current_time = 0.0;
        println!("Reset to beginning");
    }

    // Seek
    if keyboard.pressed(KeyCode::ArrowLeft) {
        game_state.current_time = (game_state.current_time - 0.1).max(0.0);
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        game_state.current_time += 0.1;
    }

    // Speed adjustment
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        game_state.speed *= 1.1;
        println!("Speed: {:.2}", game_state.speed);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        game_state.speed /= 1.1;
        println!("Speed: {:.2}", game_state.speed);
    }
}

fn create_empty_chart() -> Chart {
    Chart {
        file_version: 0,
        songs_name: String::new(),
        themes: vec![chart::Theme {
            colors_list: vec![
                chart::Color { r: 255, g: 255, b: 255, a: 255 },
                chart::Color { r: 254, g: 212, b: 212, a: 255 },
                chart::Color { r: 217, g: 217, b: 217, a: 255 },
            ],
        }],
        challenge_times: vec![],
        bpm: 120.0,
        bpm_shifts: vec![chart::BpmShift {
            time: 0.0,
            value: 1.0,
            ease_type: 0,
            floor_position: 0.0,
        }],
        offset: 0.0,
        lines: vec![],
        canvas_moves: vec![chart::CanvasMove {
            index: 0,
            x_position_key_points: vec![chart::KeyPoint {
                time: 0.0,
                value: 0.0,
                ease_type: 0,
                floor_position: 0.0,
                fp: Some(0.0),
            }],
            speed_key_points: vec![chart::KeyPoint {
                time: 0.0,
                value: 1.0,
                ease_type: 0,
                floor_position: 0.0,
                fp: Some(0.0),
            }],
        }],
        camera_move: chart::CameraMove {
            scale_key_points: vec![chart::KeyPoint {
                time: 0.0,
                value: 1.0,
                ease_type: 0,
                floor_position: 0.0,
                fp: None,
            }],
            x_position_key_points: vec![chart::KeyPoint {
                time: 0.0,
                value: 0.0,
                ease_type: 0,
                floor_position: 0.0,
                fp: None,
            }],
        },
    }
}
