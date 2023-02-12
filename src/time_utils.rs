use std::time::SystemTime;

const MILLIS_IN_DAY: u128 = 86_400_000u128;

pub fn get_current_day() -> u128 {
    let current_timestamp = get_current_timestamp();
    return current_timestamp - (current_timestamp % MILLIS_IN_DAY);
}

pub fn get_current_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
}
