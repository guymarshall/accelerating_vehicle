#![forbid(unsafe_code)]

use std::time::{Duration, Instant};
use crate::math::mps_to_mph;

mod user_input;
mod math;

fn main() {
    println!("This program works out the top speed of a vehicle with a given mass and power output");
    let mass: f64 = user_input::input("Mass (kg):");
    let input_power: f64 = user_input::input("Power (W):");

    let start_time: Instant = Instant::now();

    let air_density: f64 = 1.225;
    let drag_coefficient: f64 = 0.3;
    let frontal_area: f64 = 0.85 * 1.89 * 1.46;

    let delta_t: f64 = 1.0;

    let mut velocity: f64 = 0.0;
    let mut time_counter: f64 = 1.0;
    let mut finished: bool = false;

    while !finished {
        let air_resistance_force: f64 = ((air_density * drag_coefficient * frontal_area) / 2.0) * velocity * velocity;
        let delta_ke: f64 = delta_t * (input_power - (air_resistance_force * velocity));
        let new_velocity: f64 = f64::sqrt(velocity * velocity) + (2.0 * delta_ke / mass);

        println!("Time: {} seconds, Velocity: {} mph%n", time_counter, mps_to_mph(new_velocity));

        if new_velocity - velocity < 0.000001 {
            finished = true;
        }

        velocity = new_velocity;
        time_counter += delta_t;
    }

    let end_time: Instant = Instant::now();
    let time_taken: Duration = end_time - start_time;
    println!("This program took {:?} seconds to calculate.", time_taken);
}