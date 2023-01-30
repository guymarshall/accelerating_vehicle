mod user_input;

fn main() {
    println!("Hello, world!");
}

/*
package com.marshall.guy;

import java.util.Scanner;

public class App
{
    static Scanner scanner = new Scanner(System.in);

    public static double mpsToMph(double speedInMps)
    {
        return speedInMps * 3600.0 / 1600.0;
    }

    public static double input(String prompt)
    {
        System.out.print(prompt);
        return scanner.nextDouble();
    }

    public static void main(String[] args)
    {
        System.out.println("This program works out the top speed of a vehicle with a given mass and power output");
        double mass = input("Mass (kg): ");
        double inputPower = input("Power (W): ");

        long startTime = System.nanoTime();

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

        double timeTaken = (double)(System.nanoTime() - startTime) / 1_000_000_000;
        System.out.printf("This program took %f seconds to calculate.", timeTaken);
    }
}
*/