use rand::{self, Rng};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn canonical_random_number() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_number_in_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

