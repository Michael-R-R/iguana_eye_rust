use std::time::Instant;

pub struct Time {
    pub last_time: Instant,
    pub dt: f32,
}

impl Time {
    pub fn new() -> Self {
        Self { 
            last_time: Instant::now(), 
            dt: 0.0 
        }
    }

    pub fn update(&mut self) -> f32 {
        let curr_now = Instant::now();
        self.dt = (curr_now - self.last_time).as_secs_f32();
        self.last_time = curr_now;

        return self.dt
    }
}