use crate::chart::{BpmShift, KeyPoint};
use crate::easing::apply_ease;

/// Convert seconds to tick based on BPM shifts
pub fn seconds_to_tick(seconds: f64, bpm_shifts: &[BpmShift], base_bpm: f64) -> f64 {
    if bpm_shifts.is_empty() {
        return seconds / (60.0 / base_bpm);
    }

    let first = &bpm_shifts[0];
    if seconds <= first.floor_position {
        return seconds / (60.0 / (base_bpm * first.value));
    }

    for i in 1..bpm_shifts.len() {
        let curr = &bpm_shifts[i];
        let prev = &bpm_shifts[i - 1];

        if seconds <= curr.floor_position {
            let tick_start = prev.time;
            let tick_end = curr.time;
            let time_start = prev.floor_position;
            let time_end = curr.floor_position;

            let ratio = (seconds - time_start) / (time_end - time_start);
            return tick_start + ratio * (tick_end - tick_start);
        }
    }

    // Beyond last shift
    let last = &bpm_shifts[bpm_shifts.len() - 1];
    let extra_seconds = seconds - last.floor_position;
    let extra_ticks = extra_seconds / (60.0 / (base_bpm * last.value));
    last.time + extra_ticks
}

/// Convert tick to seconds based on BPM shifts
pub fn tick_to_seconds(tick: f64, bpm_shifts: &[BpmShift], base_bpm: f64) -> f64 {
    if bpm_shifts.is_empty() {
        return tick * (60.0 / base_bpm);
    }

    let first = &bpm_shifts[0];
    if tick <= first.time {
        return tick * (60.0 / (base_bpm * first.value));
    }

    for i in 1..bpm_shifts.len() {
        let curr = &bpm_shifts[i];
        if tick <= curr.time {
            let prev = &bpm_shifts[i - 1];
            let ratio = (tick - prev.time) / (curr.time - prev.time);
            let sec_start = prev.floor_position;
            let sec_end = curr.floor_position;
            return sec_start + ratio * (sec_end - sec_start);
        }
    }

    // Beyond last shift
    let last = &bpm_shifts[bpm_shifts.len() - 1];
    let extra_ticks = tick - last.time;
    let extra_seconds = extra_ticks * (60.0 / (base_bpm * last.value));
    last.floor_position + extra_seconds
}

/// Find interpolated value from keypoint events at given tick
pub fn find_value(tick: f64, events: &[KeyPoint]) -> f64 {
    if events.is_empty() {
        return 0.0;
    }

    if events.len() == 1 {
        return if tick >= events[0].time {
            events[0].value
        } else {
            0.0
        };
    }

    let last_event = &events[events.len() - 1];
    if tick > last_event.time {
        return last_event.value;
    }

    // Binary search
    let mut left = 0;
    let mut right = events.len() - 1;
    let mut event1: Option<&KeyPoint> = None;
    let mut event2: Option<&KeyPoint> = None;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_event = &events[mid];

        if (mid_event.time - tick).abs() < f64::EPSILON {
            return mid_event.value;
        } else if mid_event.time < tick {
            event1 = Some(mid_event);
            left = mid + 1;
        } else {
            event2 = Some(mid_event);
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    if let (Some(e1), Some(e2)) = (event1, event2) {
        let t = (tick - e1.time) / (e2.time - e1.time);
        let ease_value = apply_ease(e1.ease_type, t as f32) as f64;
        e1.value + (e2.value - e1.value) * ease_value
    } else {
        0.0
    }
}

/// Recalculate floor positions for speed keypoints
pub fn recalculate_fps(speed_key_points: &mut [KeyPoint], bpm_shifts: &[BpmShift], base_bpm: f64) {
    if speed_key_points.is_empty() {
        return;
    }

    speed_key_points[0].fp = Some(0.0);

    for i in 1..speed_key_points.len() {
        let prev_time = speed_key_points[i - 1].time;
        let prev_value = speed_key_points[i - 1].value;
        let prev_fp = speed_key_points[i - 1].fp.unwrap_or(0.0);

        let curr_time = speed_key_points[i].time;
        let time_diff =
            tick_to_seconds(curr_time, bpm_shifts, base_bpm) - tick_to_seconds(prev_time, bpm_shifts, base_bpm);

        speed_key_points[i].fp = Some(prev_fp + prev_value * time_diff);
    }
}

/// Calculate floor position from speed keypoints at given time (in seconds)
pub fn speed_to_fp(timer: f64, speed_key_points: &[KeyPoint], bpm_shifts: &[BpmShift], base_bpm: f64) -> f64 {
    if speed_key_points.is_empty() {
        return 0.0;
    }

    // Binary search for the keypoint
    let mut left = 0;
    let mut right = speed_key_points.len() - 1;
    let mut target_index = right;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_time = tick_to_seconds(speed_key_points[mid].time, bpm_shifts, base_bpm);

        if mid_time <= timer {
            target_index = mid;
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    let current = &speed_key_points[target_index];
    let current_time = tick_to_seconds(current.time, bpm_shifts, base_bpm);
    let t2 = timer - current_time;

    current.fp.unwrap_or(0.0) + t2 * current.value
}
