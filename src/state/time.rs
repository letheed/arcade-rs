use time_crate::{Duration, SteadyTime};

pub struct Time {
    // program_start: SteadyTime,
    previous_loop: SteadyTime,
    pub this_loop: SteadyTime,
    pub zero: Duration,
    pub elapsed: Duration,
    pub elapsed_ms: f64,
}

impl Time {
    pub fn init() -> Self {
        let now = SteadyTime::now();
        Time {
            // program_start: now,
            previous_loop: now,
            this_loop: now,
            zero: Duration::zero(),
            elapsed: Duration::zero(),
            elapsed_ms: 0.,
        }
    }

    pub fn new_loop(&mut self) {
        self.previous_loop = self.this_loop;
        self.this_loop = SteadyTime::now();
        self.elapsed = self.this_loop - self.previous_loop;
        self.elapsed_ms = self.elapsed.num_microseconds().unwrap() as f64;
    }
}
