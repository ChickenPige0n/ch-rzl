use serde::{Deserialize, Serialize};

/// RGBA color representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn to_bevy_color(&self) -> bevy::prelude::Color {
        bevy::prelude::Color::srgba(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8,
            a: (self.a as f32 + (other.a as f32 - self.a as f32) * t).round() as u8,
        }
    }

    pub fn mix(&self, other: &Color) -> Color {
        if other.a == 0 {
            return *self;
        }
        if other.a == 255 {
            return Color {
                r: other.r,
                g: other.g,
                b: other.b,
                a: self.a,
            };
        }
        let mix_ratio = other.a as f32 / 255.0;
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * mix_ratio).round() as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * mix_ratio).round() as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * mix_ratio).round() as u8,
            a: self.a,
        }
    }
}

/// Theme with color list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    #[serde(rename = "colorsList")]
    pub colors_list: Vec<Color>,
}

/// Challenge time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeTime {
    #[serde(rename = "checkPoint")]
    pub check_point: f64,
    pub start: f64,
    pub end: f64,
    #[serde(rename = "transTime")]
    pub trans_time: f64,
}

/// BPM shift point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpmShift {
    pub time: f64,
    pub value: f64,
    #[serde(rename = "easeType")]
    pub ease_type: u8,
    #[serde(rename = "floorPosition")]
    pub floor_position: f64,
}

/// Line point (keyframe)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinePoint {
    pub time: f64,
    #[serde(rename = "xPosition")]
    pub x_position: f64,
    pub color: Color,
    #[serde(rename = "easeType")]
    pub ease_type: u8,
    #[serde(rename = "canvasIndex")]
    pub canvas_index: usize,
    #[serde(rename = "floorPosition")]
    pub floor_position: f64,
    /// Cached floor position value (computed at runtime)
    #[serde(skip)]
    pub fp: Option<f64>,
    /// Cached mixed color (computed at runtime)
    #[serde(skip)]
    #[allow(dead_code)]
    pub mix_color: Option<Color>,
}

/// Note types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NoteType {
    Tap = 0,
    Drag = 1,
    Hold = 2,
}

impl From<u8> for NoteType {
    fn from(value: u8) -> Self {
        match value {
            0 => NoteType::Tap,
            1 => NoteType::Drag,
            2 => NoteType::Hold,
            _ => NoteType::Tap,
        }
    }
}

/// Note in the chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "type")]
    pub note_type: u8,
    pub time: f64,
    #[serde(rename = "floorPosition")]
    pub floor_position: f64,
    #[serde(rename = "otherInformations", default)]
    pub other_informations: Vec<f64>,
}

/// Color transition event for judge ring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeRingColor {
    #[serde(rename = "startColor")]
    pub start_color: Color,
    #[serde(rename = "endColor")]
    pub end_color: Color,
    pub time: f64,
}

/// Color transition event for line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineColor {
    #[serde(rename = "startColor")]
    pub start_color: Color,
    #[serde(rename = "endColor")]
    pub end_color: Color,
    pub time: f64,
}

/// Line containing points and notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    #[serde(rename = "linePoints")]
    pub line_points: Vec<LinePoint>,
    pub notes: Vec<Note>,
    #[serde(rename = "judgeRingColor")]
    pub judge_ring_color: Vec<JudgeRingColor>,
    #[serde(rename = "lineColor", default)]
    pub line_color: Vec<LineColor>,
}

/// Keyframe for canvas movement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPoint {
    pub time: f64,
    pub value: f64,
    #[serde(rename = "easeType")]
    pub ease_type: u8,
    #[serde(rename = "floorPosition")]
    pub floor_position: f64,
    /// Cached floor position (computed at runtime)
    #[serde(skip)]
    pub fp: Option<f64>,
}

/// Canvas movement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasMove {
    pub index: usize,
    #[serde(rename = "xPositionKeyPoints")]
    pub x_position_key_points: Vec<KeyPoint>,
    #[serde(rename = "speedKeyPoints")]
    pub speed_key_points: Vec<KeyPoint>,
}

/// Camera movement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraMove {
    #[serde(rename = "scaleKeyPoints")]
    pub scale_key_points: Vec<KeyPoint>,
    #[serde(rename = "xPositionKeyPoints")]
    pub x_position_key_points: Vec<KeyPoint>,
}

/// Main chart data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    #[serde(rename = "fileVersion")]
    pub file_version: u32,
    #[serde(rename = "songsName", default)]
    pub songs_name: String,
    pub themes: Vec<Theme>,
    #[serde(rename = "challengeTimes")]
    pub challenge_times: Vec<ChallengeTime>,
    #[serde(rename = "bPM")]
    pub bpm: f64,
    #[serde(rename = "bpmShifts")]
    pub bpm_shifts: Vec<BpmShift>,
    pub offset: f64,
    pub lines: Vec<Line>,
    #[serde(rename = "canvasMoves")]
    pub canvas_moves: Vec<CanvasMove>,
    #[serde(rename = "cameraMove")]
    pub camera_move: CameraMove,
}

impl Chart {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
