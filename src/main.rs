#![forbid(unsafe_code)]

use std::time::{Duration, Instant};

mod user_input;
mod math;

fn main() {
    println!("This program works out the top speed of a vehicle with a given mass and power output");
    let mass: f64 = user_input::input("Mass (kg):");
    let input_power: f64 = user_input::input("Power (W):");

    let start_time: Instant = Instant::now();

    let end_time: Instant = Instant::now();
    let time_taken: Duration = end_time - start_time;
    println!("This program took {:?} seconds to calculate.", time_taken);
}

/*
double airDensity = 1.225;
double dragCoefficient = 0.3;
double frontalArea = 0.85 * 1.89 * 1.46;

int deltaT = 1;

double velocity = 0.0;
int timeCounter = 1;
boolean finished = false;

while (!finished)
{
    double airResistanceForce = ((airDensity * dragCoefficient * frontalArea) / 2) * Math.pow(velocity, 2);
    double deltaKE = deltaT * (inputPower - (airResistanceForce * velocity));
    double newVelocity = Math.sqrt(Math.pow(velocity, 2) + (2 * deltaKE / mass));

    System.out.printf("Time: %d seconds, Velocity: %f mph%n", timeCounter, mpsToMph(newVelocity));

    if (newVelocity - velocity < 0.000001)
    {
        finished = true;
    }

    velocity = newVelocity;
    timeCounter += deltaT;
}
*/