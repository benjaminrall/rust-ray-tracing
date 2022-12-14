pub const INFINITY: f64 = f64::INFINITY;    // Storing value for infinity
pub const PI: f64 = std::f64::consts::PI;   // Storing value for PI

/// Converts a given value in degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

/// Generates a random double between 0 and 1
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

/// Generates a random double in a given range
pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

/// Generates a random integer in a given range
pub fn random_int(min: i32, max: i32) -> i32 {
    random_range(min as f64, (max + 1) as f64) as i32
}

/// Clamps a given value between a minimum and maximum
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}