#![forbid(unsafe_code)]

/*
public static double mpsToMph(double speedInMps)
{
    return speedInMps * 3600.0 / 1600.0;
}

public static double input(String prompt)
{
    System.out.print(prompt);
    return scanner.nextDouble();
}
*/

pub fn mps_to_mph(speed_in_mps: f64) -> f64 {
    speed_in_mps * 3600.0 / 1600.0
}