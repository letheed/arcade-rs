use assets::Assets;
use sdl::render::Renderer;
use state::State;
use self::pattern::SlidingWindowPattern;
use self::ship::Ship;

mod pattern;
mod primitives;
mod ship;

pub struct Game<'a> {
    bg_back: SlidingWindowPattern<'a>,
    // bg_middle: SlidingWindowPattern<'a>,
    bg_front: SlidingWindowPattern<'a>,
    player: Ship<'a>,
}

impl<'a> Game<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        Game {
            bg_back: SlidingWindowPattern::new(&assets.star_bg, -10.),
            // bg_middle: SlidingWindowPattern::new(&assets.star_mg, -200.),
            bg_front: SlidingWindowPattern::new(&assets.star_fg, -12.),
            player: Ship::new(&assets.spaceship)
        }
    }

    pub fn render(&mut self, state: &State, renderer: &mut Renderer) {
        self.bg_back.render(state, renderer);
        // self.bg_middle.render(state, renderer);
        self.bg_front.render(state, renderer);
        self.player.render(state, renderer);
    }
}
