#![forbid(unsafe_code)]

pub fn mps_to_mph(speed_in_mps: f64) -> f64 {
    speed_in_mps * 3600.0 / 1600.0
}