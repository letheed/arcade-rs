extern crate sdl2 as sdl;
extern crate time as time_crate;

use assets::Assets;
use config::Config;
use context::SdlContext;
use state::State;
use state::time::Time;
use view::View;
use view::ViewAction::*;

const CRATE_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

#[macro_use]
mod macros;
mod assets;
mod config;
mod context;
mod game;
mod state;
mod view;

fn main() {
    let config = Config::load();
    let (_sdl, event_pump, mut renderer) = SdlContext::init(&config);
    let assets = Assets::load(&renderer);
    let mut state = State::init(config, event_pump);
    let mut view = View::init(&assets);
    loop {
        state.time.new_loop();
        state.pump_events();
        match view.render(&state, &mut renderer) {
            Present => renderer.present(),
            Quit => break,
        }
        // print_fps(&state.time);
        state.pause_on_demand();
    }
}

#[allow(dead_code)]
fn print_fps(time: &Time) {
    let fps = 1. / time.elapsed_ms * 1E+6;
    println!("{}", fps);
}
