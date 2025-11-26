use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::chart::Color as ChartColor;

/// Default white color constant
pub const WHITE_COLOR: ChartColor = ChartColor { r: 255, g: 255, b: 255, a: 255 };

/// Line width for judge ring
pub const JUDGE_RING_LINE_WIDTH: f32 = 5.0;
use crate::easing::apply_ease;
use crate::game::{
    calculate_mixed_color, compute_line_point, get_current_judge_ring_color, update_canvas_states,
    ComputedLinePoint, GameState, NoteState,
};
use crate::timing::{speed_to_fp, tick_to_seconds};

/// Window configuration
pub const WINDOW_WIDTH: f32 = 1080.0;
pub const WINDOW_HEIGHT: f32 = 1920.0;
#[allow(dead_code)]
pub const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

/// Marker component for game entities
#[derive(Component)]
pub struct GameEntity;

/// Marker for background
#[derive(Component)]
pub struct Background;

/// Marker for line segments
#[derive(Component)]
#[allow(dead_code)]
pub struct LineSegment {
    pub line_index: usize,
    pub point_index: usize,
}

/// Marker for notes
#[derive(Component)]
#[allow(dead_code)]
pub struct NoteEntity {
    pub line_index: usize,
    pub note_index: usize,
}

/// Marker for judge ring
#[derive(Component)]
#[allow(dead_code)]
pub struct JudgeRing {
    pub line_index: usize,
}

/// Marker for hit effect
#[derive(Component)]
#[allow(dead_code)]
pub struct HitEffect {
    pub start_time: f64,
    pub x: f32,
    pub color: ChartColor,
}

/// Marker for combo text
#[derive(Component)]
#[allow(dead_code)]
pub struct ComboText;

/// Note states resource
#[derive(Resource, Default)]
pub struct NoteStates {
    pub states: Vec<Vec<NoteState>>,
}

/// Hit count resource
#[derive(Resource, Default)]
pub struct HitCount(pub u32);

/// Plugin for rendering
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoteStates>()
            .init_resource::<HitCount>()
            .add_systems(Update, (
                update_game_time,
                update_rendering,
            ).chain());
    }
}

/// Update game time based on audio or frame time
fn update_game_time(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.is_playing {
        game_state.current_time += time.delta_seconds_f64();
    }
}

/// Main rendering update system
fn update_rendering(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut note_states: ResMut<NoteStates>,
    mut hit_count: ResMut<HitCount>,
    mut background_query: Query<&mut Sprite, With<Background>>,
    game_entities: Query<Entity, With<GameEntity>>,
) {
    // Update canvas states
    update_canvas_states(&mut game_state);

    let tick = game_state.current_tick();
    let _timer = game_state.current_time;
    let scale = game_state.camera_scale() as f32;
    let screen_width = WINDOW_WIDTH;
    let screen_height = WINDOW_HEIGHT;

    // Update background color
    let bg_color = game_state.get_theme_color(0);
    for mut sprite in background_query.iter_mut() {
        sprite.color = bg_color.to_bevy_color();
    }

    // Clear old dynamic entities (we'll rebuild each frame for simplicity)
    for entity in game_entities.iter() {
        commands.entity(entity).despawn();
    }

    // Draw lines
    for (_line_idx, line) in game_state.chart.lines.iter().enumerate() {
        let line_points = &line.line_points;
        let line_color = &line.line_color;

        for i in 0..line_points.len() {
            let point = &line_points[i];

            if point.canvas_index >= game_state.canvas_states.len() {
                continue;
            }

            let canvas_state = &game_state.canvas_states[point.canvas_index];
            let canvas_fp = canvas_state.fp;
            let canvas_x = canvas_state.x as f32 * screen_width;

            let computed = compute_line_point(
                &game_state,
                point,
                canvas_fp,
                canvas_x as f64,
                line_color,
                screen_width as f64,
                screen_height as f64,
            );

            // Skip if off screen
            if computed.y < -screen_height as f64 * 2.0 {
                continue;
            }

            // Draw line segment to next point
            if i + 1 < line_points.len() {
                let next_point = &line_points[i + 1];
                if next_point.canvas_index >= game_state.canvas_states.len() {
                    continue;
                }

                let next_canvas_state = &game_state.canvas_states[next_point.canvas_index];
                let next_computed = compute_line_point(
                    &game_state,
                    next_point,
                    next_canvas_state.fp,
                    next_canvas_state.x as f32 as f64 * screen_width as f64,
                    line_color,
                    screen_width as f64,
                    screen_height as f64,
                );

                // Skip if completely off screen
                if next_computed.y > screen_height as f64 * 2.0 {
                    continue;
                }

                // Draw line segment
                draw_line_segment(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &computed,
                    &next_computed,
                    scale,
                );
            }

            // Draw judge ring if within time range
            if tick >= point.time {
                if i + 1 < line_points.len() {
                    let next_point = &line_points[i + 1];
                    if tick < next_point.time {
                        let progress = (tick - point.time) / (next_point.time - point.time);
                        let ease_value = apply_ease(point.ease_type, progress as f32);

                        let next_canvas_state = &game_state.canvas_states[next_point.canvas_index];
                        let next_computed = compute_line_point(
                            &game_state,
                            next_point,
                            next_canvas_state.fp,
                            next_canvas_state.x as f32 as f64 * screen_width as f64,
                            line_color,
                            screen_width as f64,
                            screen_height as f64,
                        );

                        let ring_x = computed.x + ease_value as f64 * (next_computed.x - computed.x);

                        if let Some(ring_color) = get_current_judge_ring_color(&line.judge_ring_color, tick) {
                            let mixed_color = calculate_mixed_color(tick, &ring_color, line_color);
                            draw_judge_ring(
                                &mut commands,
                                &mut meshes,
                                &mut materials,
                                ring_x as f32,
                                0.0,
                                30.0 * scale,
                                mixed_color,
                            );
                        }
                    }
                }
            }
        }
    }

    // Initialize note states if needed
    if note_states.states.len() != game_state.chart.lines.len() {
        note_states.states.clear();
        for line in &game_state.chart.lines {
            note_states.states.push(vec![NoteState::default(); line.notes.len()]);
        }
    }

    // Draw notes
    for (line_idx, line) in game_state.chart.lines.iter().enumerate() {
        let line_points = &line.line_points;

        for (note_idx, note) in line.notes.iter().enumerate() {
            if note_idx >= note_states.states[line_idx].len() {
                continue;
            }

            let note_state = &mut note_states.states[line_idx][note_idx];

            // Skip if already hit (for non-hold notes)
            if note_state.is_hit && tick > note.time && note.note_type != 2 {
                continue;
            }

            // Find the line point for this note's time
            let (point, next_point) = find_line_points_for_time(line_points, note.time);

            if point.canvas_index >= game_state.canvas_states.len() {
                continue;
            }

            let canvas_state = &game_state.canvas_states[point.canvas_index];

            // Calculate note position
            let note_fp = speed_to_fp(
                tick_to_seconds(note.time, &game_state.chart.bpm_shifts, game_state.chart.bpm),
                &game_state.chart.canvas_moves[point.canvas_index].speed_key_points,
                &game_state.chart.bpm_shifts,
                game_state.chart.bpm,
            );

            let point_x = point.x_position * scale as f64 * screen_width as f64 + canvas_state.x * screen_width as f64;
            let next_point_x = if let Some(np) = next_point {
                let next_canvas = &game_state.canvas_states[np.canvas_index.min(game_state.canvas_states.len() - 1)];
                np.x_position * scale as f64 * screen_width as f64 + next_canvas.x * screen_width as f64
            } else {
                point_x
            };

            let t = if let Some(np) = next_point {
                if np.time != point.time {
                    (note.time - point.time) / (np.time - point.time)
                } else {
                    0.0
                }
            } else {
                0.0
            };

            let ease_value = apply_ease(point.ease_type, t as f32) as f64;
            let note_x = point_x + ease_value * (next_point_x - point_x);

            let note_y = if note.note_type == 2 && tick >= note.time {
                0.0
            } else {
                -(note_fp - canvas_state.fp) * screen_height as f64 * game_state.speed * scale as f64
            };

            // Check if note should be hit
            if !note_state.is_hit && tick >= note.time {
                note_state.is_hit = true;
                note_state.is_play_hit = true;
                hit_count.0 += 1;
            }

            // Reset if going backward in time
            if tick < note.time {
                note_state.is_hit = false;
                note_state.is_play_hit = false;
            }

            // Get note color
            let note_color = if note.note_type == 1 {
                WHITE_COLOR
            } else {
                game_state.get_theme_color(1)
            };

            // Draw note
            let note_size = 20.0 * scale;
            draw_note(
                &mut commands,
                &mut meshes,
                &mut materials,
                note_x as f32,
                note_y as f32,
                note_size,
                note_color,
            );

            // Draw hold body if hold note
            if note.note_type == 2 && !note.other_informations.is_empty() {
                let end_time = note.other_informations[0];
                if tick <= end_time {
                    let end_canvas_idx = if note.other_informations.len() > 1 {
                        note.other_informations[1] as usize
                    } else {
                        point.canvas_index
                    };

                    if end_canvas_idx < game_state.canvas_states.len() {
                        let end_fp = speed_to_fp(
                            tick_to_seconds(end_time, &game_state.chart.bpm_shifts, game_state.chart.bpm),
                            &game_state.chart.canvas_moves[end_canvas_idx].speed_key_points,
                            &game_state.chart.bpm_shifts,
                            game_state.chart.bpm,
                        );

                        let end_canvas = &game_state.canvas_states[end_canvas_idx];
                        let end_y = (end_canvas.fp - end_fp) * screen_height as f64 * game_state.speed * scale as f64;
                        let height = (end_y - note_y) as f32;

                        draw_hold_body(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            note_x as f32,
                            note_y as f32,
                            10.0 * scale,
                            height,
                            note_color,
                        );
                    }
                }
            }
        }
    }
}

fn find_line_points_for_time<'a>(
    points: &'a [crate::chart::LinePoint],
    time: f64,
) -> (&'a crate::chart::LinePoint, Option<&'a crate::chart::LinePoint>) {
    let mut left = 0;
    let mut right = points.len().saturating_sub(1);
    let mut target_index = right;

    while left <= right {
        let mid = (left + right) / 2;
        if points[mid].time <= time {
            target_index = mid;
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    let point = &points[target_index];
    let next_point = if target_index + 1 < points.len() {
        Some(&points[target_index + 1])
    } else {
        None
    };

    (point, next_point)
}

fn draw_line_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start: &ComputedLinePoint,
    end: &ComputedLinePoint,
    scale: f32,
) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let length = ((dx * dx + dy * dy) as f32).sqrt();

    if length < 0.1 {
        return;
    }

    let angle = (dy as f32).atan2(dx as f32);
    let mid_x = (start.x + end.x) / 2.0;
    let mid_y = (start.y + end.y) / 2.0;

    let color = start.color.to_bevy_color();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(length, 3.0 * scale)).into(),
            material: materials.add(color),
            transform: Transform::from_xyz(mid_x as f32, mid_y as f32, 1.0)
                .with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
        GameEntity,
    ));
}

fn draw_judge_ring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    size: f32,
    color: ChartColor,
) {
    let bevy_color = color.to_bevy_color();

    // Draw ring as 4 rectangles (top, bottom, left, right)
    let half_size = size / 2.0;
    let line_width = JUDGE_RING_LINE_WIDTH;

    // Top
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(size, line_width)).into(),
            material: materials.add(bevy_color),
            transform: Transform::from_xyz(x, y + half_size, 2.0),
            ..default()
        },
        GameEntity,
    ));

    // Bottom
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(size, line_width)).into(),
            material: materials.add(bevy_color),
            transform: Transform::from_xyz(x, y - half_size, 2.0),
            ..default()
        },
        GameEntity,
    ));

    // Left
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(line_width, size)).into(),
            material: materials.add(bevy_color),
            transform: Transform::from_xyz(x - half_size, y, 2.0),
            ..default()
        },
        GameEntity,
    ));

    // Right
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(line_width, size)).into(),
            material: materials.add(bevy_color),
            transform: Transform::from_xyz(x + half_size, y, 2.0),
            ..default()
        },
        GameEntity,
    ));
}

fn draw_note(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    size: f32,
    color: ChartColor,
) {
    let bevy_color = color.to_bevy_color();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(size, size)).into(),
            material: materials.add(bevy_color),
            transform: Transform::from_xyz(x, y, 3.0),
            ..default()
        },
        GameEntity,
    ));

    // Draw border
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(size + 3.0, size + 3.0)).into(),
            material: materials.add(Color::BLACK),
            transform: Transform::from_xyz(x, y, 2.9),
            ..default()
        },
        GameEntity,
    ));
}

fn draw_hold_body(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: ChartColor,
) {
    let bevy_color = color.to_bevy_color();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(width, height.abs())).into(),
            material: materials.add(bevy_color),
            transform: Transform::from_xyz(x, y + height / 2.0, 2.5),
            ..default()
        },
        GameEntity,
    ));
}
