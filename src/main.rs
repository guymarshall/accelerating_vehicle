use std::{thread, time::Duration};

mod user_input;

fn main() {
    println!(
        "This program works out the top speed of a vehicle with a given mass and power output."
    );
    let mass: f64 = user_input::input("Mass (kg):");

    let input_power: f64 = user_input::input("Power (W): ");

    let air_density: f64 = 1.225; // 15C at sea level
    let drag_coefficient: f64 = 0.3; // normal car
    let frontal_area: f64 = 0.85 * 1.89 * 1.46; // normal car

    let mut velocity: f64 = 0.1;
    let delta_time: f64 = 0.00001;

    let acceleration_threshold: f64 = 0.1;
    let mut acceleration: f64;
    let mut time: f64 = 0.0;
    let mut latest_time: i32 = 0;

    loop {
        let air_resistance_force: f64 =
            ((air_density * drag_coefficient * frontal_area) / 2.0) * velocity * velocity;
        let power: f64 = input_power - (air_resistance_force * velocity);
        let velocity_as_kinetic_energy: f64 = 0.5 * mass * velocity * velocity;
        let kinetic_energy: f64 = velocity_as_kinetic_energy + (power * delta_time);
        let new_velocity: f64 = ((2.0 * kinetic_energy) / mass).sqrt();
        acceleration = (new_velocity - velocity) / delta_time;
        velocity = new_velocity;

        let new_time_different: bool = time as i32 != latest_time;
        if new_time_different {
            latest_time = time as i32;
            thread::sleep(Duration::from_millis(1000));
            println!("Time: {} seconds, Velocity: {} m/s.", latest_time, velocity,);
        }

        time += delta_time;

        if acceleration < acceleration_threshold {
            break;
        }
    }
}
