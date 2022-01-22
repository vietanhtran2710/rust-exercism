// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars: f64 = (speed as f64) * 221.0;
    if speed <= 4 {
        return cars;
    }
    else if speed <= 8 {
        return cars * 90.0 / 100.0;
    }
    else {
        return cars as f64 * 77.0 / 100.0;
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) as u32) / 60
}
