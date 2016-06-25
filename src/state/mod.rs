use config::Config;
use sdl::EventPump;
use std::{mem, thread};
use time_crate::SteadyTime;
use self::input::{Events, InputState};
use self::time::Time;

pub mod input;
pub mod time;

pub struct State {
    pub config: Config,
    pub events: Events,
    pub input: InputState,
    pub definition: (u32, u32),
    pub definitionf64: (f64, f64),
    pub time: Time,
}

impl State {
    pub fn init(config: Config, event_pump: EventPump) -> Self {
        let events = Events::new(event_pump);
        let input = InputState::new();
        let time = Time::init();
        State::new(config, events, input, time)
    }

    pub fn pause_on_demand(&self) {
        let time_to_frame = self.config.frame_lapse - (SteadyTime::now() - self.time.this_loop);
        if time_to_frame > self.time.zero {
            thread::sleep(unsafe { mem::transmute(time_to_frame) });
        }
    }
}

impl State {
    fn new(config: Config, events: Events, input: InputState, time:Time) -> Self {
        let definition = config.definition;
        let definitionf64 = (definition.0 as f64, definition.1 as f64);
        State {
            config: config,
            events: events,
            input: input,
            definition: definition,
            definitionf64: definitionf64,
            time: time,
        }
    }
}
