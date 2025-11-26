/// Easing functions for animations
/// Matches the 19 easing types from the original JavaScript implementation

pub fn linear(x: f32) -> f32 {
    x
}

pub fn ease_in_quad(x: f32) -> f32 {
    x * x
}

pub fn ease_out_quad(x: f32) -> f32 {
    1.0 - (1.0 - x) * (1.0 - x)
}

pub fn ease_in_out_quad(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - (-2.0 * x + 2.0_f32).powi(2) / 2.0
    }
}

pub fn ease_in_cubic(x: f32) -> f32 {
    x * x * x
}

pub fn ease_out_cubic(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(3)
}

pub fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0_f32).powi(3) / 2.0
    }
}

pub fn ease_in_quart(x: f32) -> f32 {
    x * x * x * x
}

pub fn ease_out_quart(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(4)
}

pub fn ease_in_out_quart(x: f32) -> f32 {
    if x < 0.5 {
        8.0 * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0_f32).powi(4) / 2.0
    }
}

pub fn ease_in_quint(x: f32) -> f32 {
    x * x * x * x * x
}

pub fn ease_out_quint(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(5)
}

pub fn ease_in_out_quint(x: f32) -> f32 {
    if x < 0.5 {
        16.0 * x * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0_f32).powi(5) / 2.0
    }
}

pub fn ease_zero(_x: f32) -> f32 {
    0.0
}

pub fn ease_one(_x: f32) -> f32 {
    1.0
}

pub fn ease_in_circ(x: f32) -> f32 {
    1.0 - (1.0 - x * x).sqrt()
}

pub fn ease_out_circ(x: f32) -> f32 {
    (1.0 - (x - 1.0).powi(2)).sqrt()
}

pub fn ease_out_sine(x: f32) -> f32 {
    (x * std::f32::consts::PI / 2.0).sin()
}

pub fn ease_in_sine(x: f32) -> f32 {
    1.0 - (x * std::f32::consts::PI / 2.0).cos()
}

/// Get easing function by type index (0-18)
pub fn get_ease_func(ease_type: u8) -> fn(f32) -> f32 {
    match ease_type {
        0 => linear,
        1 => ease_in_quad,
        2 => ease_out_quad,
        3 => ease_in_out_quad,
        4 => ease_in_cubic,
        5 => ease_out_cubic,
        6 => ease_in_out_cubic,
        7 => ease_in_quart,
        8 => ease_out_quart,
        9 => ease_in_out_quart,
        10 => ease_in_quint,
        11 => ease_out_quint,
        12 => ease_in_out_quint,
        13 => ease_zero,
        14 => ease_one,
        15 => ease_in_circ,
        16 => ease_out_circ,
        17 => ease_out_sine,
        18 => ease_in_sine,
        _ => linear,
    }
}

/// Apply easing to interpolate between two values
pub fn apply_ease(ease_type: u8, t: f32) -> f32 {
    let ease_fn = get_ease_func(ease_type);
    ease_fn(t.clamp(0.0, 1.0))
}

/// Linear interpolation
#[allow(dead_code)]
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
