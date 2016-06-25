use time_crate::Duration;

pub struct Config {
    pub name: &'static str,
    pub fps_upper_bound: u8,
    pub definition: (u32, u32),
    pub frame_lapse: Duration,
}

impl Config {
    pub fn load() -> Self {
        let fps_upper_bound = 120;
        Config {
            name: "ArcadeRS Shooter",
            fps_upper_bound: fps_upper_bound,
            definition: (800, 600),
            frame_lapse: Duration::nanoseconds(1_000_000_000 / fps_upper_bound as i64),
        }
    }
}
