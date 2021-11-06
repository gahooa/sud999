

#[allow(dead_code)]
pub fn unix_timestamp_u64() -> u64 {

    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");


    return since_the_epoch.as_secs();
}


pub fn unix_timestamp_f64() -> f64 {

    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");


    return since_the_epoch.as_secs_f64();
}



