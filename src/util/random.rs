use rand::{self, Rng};

pub fn rand_signed32(lower: i32, upper: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let value: i32 = rng.gen_range(lower..upper);

    return value
}

pub fn rand_signed64(lower: i64, upper: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let value: i64 = rng.gen_range(lower..upper);

    return value
}

pub fn rand_unsigned32(lower: u32, upper: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let value: u32 = rng.gen_range(lower..upper);

    return value
}

pub fn rand_unsigned64(lower: u64, upper: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let value: u64 = rng.gen_range(lower..upper);

    return value
}

pub fn rand_float32(lower: f32, upper: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let value: f32 = rng.gen_range(lower..upper);

    return value
}

pub fn rand_float64(lower: f64, upper: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let value: f64 = rng.gen_range(lower..upper);

    return value
}