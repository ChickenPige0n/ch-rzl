use bevy::prelude::*;

use crate::chart::{Chart, Color as ChartColor, JudgeRingColor, LineColor};
use crate::timing::{find_value, recalculate_fps, seconds_to_tick, speed_to_fp, tick_to_seconds};

/// Game state resource
#[derive(Resource)]
pub struct GameState {
    pub chart: Chart,
    pub current_time: f64,
    pub is_playing: bool,
    pub speed: f64,
    pub revelation_size: f64,
    pub canvas_states: Vec<CanvasState>,
}

impl GameState {
    pub fn new(chart: Chart) -> Self {
        let canvas_count = chart.canvas_moves.len();
        let mut canvas_states = Vec::with_capacity(canvas_count);

        for _ in 0..canvas_count {
            canvas_states.push(CanvasState::default());
        }

        Self {
            chart,
            current_time: 0.0,
            is_playing: false,
            speed: (215.0 / 32.0 + 10.0) * (10.0 / 129.0),
            revelation_size: 1.0,
            canvas_states,
        }
    }

    pub fn current_tick(&self) -> f64 {
        seconds_to_tick(self.current_time, &self.chart.bpm_shifts, self.chart.bpm)
    }

    pub fn camera_scale(&self) -> f64 {
        let tick = self.current_tick();
        find_value(tick, &self.chart.camera_move.scale_key_points) * self.revelation_size
    }

    pub fn camera_move_x(&self) -> f64 {
        let tick = self.current_tick();
        find_value(tick, &self.chart.camera_move.x_position_key_points)
    }

    pub fn get_challenge_time_index(&self) -> Option<usize> {
        let tick = self.current_tick();
        for (i, ct) in self.chart.challenge_times.iter().enumerate() {
            if tick >= ct.start && tick <= ct.end {
                return Some(i + 1);
            }
        }
        None
    }

    pub fn get_theme_color(&self, index: usize) -> ChartColor {
        let theme_idx = self.get_challenge_time_index().unwrap_or(0);
        if theme_idx < self.chart.themes.len() {
            if index < self.chart.themes[theme_idx].colors_list.len() {
                return self.chart.themes[theme_idx].colors_list[index];
            }
        }
        ChartColor::default()
    }
}

/// Canvas state tracking
#[derive(Default, Clone)]
pub struct CanvasState {
    pub x: f64,
    pub fp: f64,
}

/// Initialize canvas states with recalculated floor positions
pub fn initialize_canvas_states(game_state: &mut GameState) {
    let bpm_shifts = game_state.chart.bpm_shifts.clone();
    let base_bpm = game_state.chart.bpm;

    for canvas_move in &mut game_state.chart.canvas_moves {
        recalculate_fps(&mut canvas_move.speed_key_points, &bpm_shifts, base_bpm);
    }
}

/// Update canvas positions
pub fn update_canvas_states(game_state: &mut GameState) {
    let tick = game_state.current_tick();
    let timer = game_state.current_time;
    let scale = game_state.camera_scale();
    let camera_x = game_state.camera_move_x();

    for (i, canvas_move) in game_state.chart.canvas_moves.iter().enumerate() {
        let x_value = find_value(tick, &canvas_move.x_position_key_points);
        let fp = speed_to_fp(
            timer,
            &canvas_move.speed_key_points,
            &game_state.chart.bpm_shifts,
            game_state.chart.bpm,
        );

        if i < game_state.canvas_states.len() {
            game_state.canvas_states[i].x = (x_value - camera_x) * scale;
            game_state.canvas_states[i].fp = fp;
        }
    }
}

/// Get current line color from line color events
pub fn get_current_line_color(line_color: &[LineColor], tick: f64) -> Option<ChartColor> {
    if line_color.is_empty() {
        return None;
    }

    if line_color.len() == 1 {
        return Some(line_color[0].start_color);
    }

    if tick >= line_color[line_color.len() - 1].time {
        return Some(line_color[line_color.len() - 1].end_color);
    }

    for i in 0..line_color.len() {
        let segment = &line_color[i];
        let next_time = if i + 1 < line_color.len() {
            line_color[i + 1].time
        } else {
            segment.time
        };

        if tick >= segment.time && tick < next_time {
            let duration = next_time - segment.time;
            if duration > 0.0 {
                let progress = ((tick - segment.time) / duration) as f32;
                return Some(segment.start_color.lerp(&segment.end_color, progress));
            }
            return Some(segment.start_color);
        }
    }

    Some(line_color[0].start_color)
}

/// Get current judge ring color
pub fn get_current_judge_ring_color(judge_ring_color: &[JudgeRingColor], tick: f64) -> Option<ChartColor> {
    if judge_ring_color.is_empty() {
        return None;
    }

    if judge_ring_color.len() == 1 {
        return Some(judge_ring_color[0].start_color);
    }

    if tick >= judge_ring_color[judge_ring_color.len() - 1].time {
        return Some(judge_ring_color[judge_ring_color.len() - 1].end_color);
    }

    for i in 0..judge_ring_color.len() {
        let segment = &judge_ring_color[i];
        let next_time = if i + 1 < judge_ring_color.len() {
            judge_ring_color[i + 1].time
        } else {
            segment.time
        };

        if tick >= segment.time && tick < next_time {
            let duration = next_time - segment.time;
            if duration > 0.0 {
                let progress = ((tick - segment.time) / duration) as f32;
                return Some(segment.start_color.lerp(&segment.end_color, progress));
            }
            return Some(segment.start_color);
        }
    }

    Some(judge_ring_color[0].start_color)
}

/// Calculate mixed color from point color and line color
pub fn calculate_mixed_color(
    tick: f64,
    point_color: &ChartColor,
    line_color: &[LineColor],
) -> ChartColor {
    if line_color.is_empty() {
        return *point_color;
    }

    if let Some(current_line_color) = get_current_line_color(line_color, tick) {
        point_color.mix(&current_line_color)
    } else {
        *point_color
    }
}

/// Calculate combo score
#[allow(dead_code)]
pub fn calculate_combo(combo: u32) -> u32 {
    if combo == 0 {
        0
    } else if combo <= 5 {
        combo
    } else if combo <= 8 {
        2 * combo - 5
    } else if combo <= 11 {
        3 * combo - 13
    } else {
        4 * combo - 24
    }
}

/// Line point with computed values for rendering
#[allow(dead_code)]
pub struct ComputedLinePoint {
    pub x: f64,
    pub y: f64,
    pub color: ChartColor,
    pub ease_type: u8,
}

/// Compute line point position and color
pub fn compute_line_point(
    game_state: &GameState,
    point: &crate::chart::LinePoint,
    canvas_fp: f64,
    canvas_x: f64,
    line_color: &[LineColor],
    screen_width: f64,
    screen_height: f64,
) -> ComputedLinePoint {
    let tick = game_state.current_tick();
    let scale = game_state.camera_scale();

    let point_fp = point.fp.unwrap_or_else(|| {
        speed_to_fp(
            tick_to_seconds(point.time, &game_state.chart.bpm_shifts, game_state.chart.bpm),
            &game_state.chart.canvas_moves[point.canvas_index].speed_key_points,
            &game_state.chart.bpm_shifts,
            game_state.chart.bpm,
        )
    });

    let x = point.x_position * scale * screen_width + canvas_x;
    let y = -(point_fp - canvas_fp) * screen_height * game_state.speed * scale;

    let mix_color = calculate_mixed_color(tick, &point.color, line_color);

    ComputedLinePoint {
        x,
        y,
        color: mix_color,
        ease_type: point.ease_type,
    }
}

/// Note state for tracking hit status
#[derive(Clone)]
pub struct NoteState {
    pub is_hit: bool,
    pub is_play_hit: bool,
}

impl Default for NoteState {
    fn default() -> Self {
        Self {
            is_hit: false,
            is_play_hit: false,
        }
    }
}
