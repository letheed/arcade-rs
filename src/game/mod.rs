use assets::Assets;
use sdl::render::Renderer;
use state::State;
use self::ship::Ship;

mod primitives;
mod ship;

pub struct Game<'a> {
    player: Ship<'a>,
}

impl<'a> Game<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        Game {
            player: Ship::new(&assets.ship)
        }
    }

    pub fn render(&mut self, state: &State, renderer: &mut Renderer) {
        self.player.render(state, renderer);
    }
}
