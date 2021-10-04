const CAR_PRODUCED_PER_HOUR: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        x if x <= 4 => 1.0,
        x if x <= 8 => 0.9,
        _ => 0.77,
    };

    speed as f64 * CAR_PRODUCED_PER_HOUR as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
