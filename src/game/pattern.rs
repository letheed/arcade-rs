use assets::sprite::Sprite;
use sdl::rect::Rect as SdlRect;
use sdl::render::Renderer;
use state::State;

pub struct SlidingWindowPattern<'a> {
    abs: i32,
    absf64: f64,
    speed: f64,
    sprite: &'a Sprite,
}

impl<'a> SlidingWindowPattern<'a> {
    pub fn new(sprite: &'a Sprite, speed: f64) -> Self {
        SlidingWindowPattern {
            abs: 0,
            absf64: 0.,
            speed: speed * 1E-6, // px/ms
            sprite: sprite,
        }
    }

    pub fn render(&mut self, state: &State, renderer: &mut Renderer) {
        self.update_state(state);
        let (win_width, win_height) = (state.definition.0 as i32, state.definition.1 as i32);
        let (tex_width, tex_height) = (self.sprite.width as i32, self.sprite.height as i32);
        for x in (-1..).map(|i| self.abs + i * tex_width).take_while(|&x| x < win_width) {
            for y in (0..).map(|j| j * tex_height).take_while(|&y| y < win_height) {
                let surface = SdlRect::new(x, y, tex_width as u32, tex_height as u32);
                self.sprite.render(renderer, surface);
            }
        }
    }
}

impl<'a> SlidingWindowPattern<'a> {
    fn update_state(&mut self, state: &State) {
        self.absf64 += self.speed * state.time.elapsed_ms;
        self.absf64 = modulo(self.absf64, self.sprite.width as f64);
        self.abs = self.absf64 as i32;
    }
}

#[inline]
fn modulo(n: f64, d: f64) -> f64 {
    if n < 0. { n % d + d }
    else { n % d }
}
