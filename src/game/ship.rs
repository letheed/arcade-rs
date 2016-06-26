use assets::sprite::SpriteSheet;
use sdl::render::Renderer;
use state::State;
use super::primitives::Rectangle;

pub struct Ship<'a> {
    speed: f64,
    surface: Rectangle,
    sprite_sheet: &'a SpriteSheet,
    sprite_coord: (u32, u32), // (row, col) of the sprite in the sprite sheet.
}

impl<'a> Ship<'a> {
    pub fn new(sprite: &'a SpriteSheet) -> Self {
        Ship {
            speed: 400. * 1E-6, // px/ms
            surface: Rectangle::new(64.0, 64.0, sprite.width as f64, sprite.height as f64),
            sprite_sheet: sprite,
            sprite_coord: (0, 1),
        }
    }

    pub fn render(&mut self, state: &State, renderer: &mut Renderer) {
        self.update_state(state);
        self.sprite_sheet.render(renderer, self.sprite_coord, self.surface.to_sdl());
    }
}


impl<'a> Ship<'a> {
    fn update_state(&mut self, state: &State) {
        let (dx, dy) = self.displacement(state);
        self.move_bounded(dx, dy, state.definitionf64);
        self.update_sprite_coord(dx, dy);
    }

    fn displacement(&self, state: &State) -> (f64, f64) {
        let input = &state.input;
        let travel = {
            let diagonal = (input.key_up ^ input.key_down) && (input.key_left ^ input.key_right);
            let ratio = if diagonal { 1. / 2.0f64.sqrt() } else { 1. };
            self.speed * state.time.elapsed_ms * ratio
        };
        let dx = {
            if input.key_left == input.key_right { 0. }
            else if input.key_right { travel }
            else { -travel }
        };
        let dy = {
            if input.key_up == input.key_down { 0. }
            else if input.key_down { travel }
            else { -travel }
        };
        (dx, dy)
    }

    fn move_bounded(&mut self, dx: f64, dy: f64, (width, height): (f64, f64)) {
        let x = self.surface.x + dx;
        let y = self.surface.y + dy;
        if x <= 0. { self.surface.x = 0.; }
        else if x + self.surface.w >= width { self.surface.x = width - self.surface.w; }
        else { self.surface.x = x; }
        if y <= 0. { self.surface.y = 0.; }
        else if y + self.surface.h >= height { self.surface.y = height - self.surface.h; }
        else { self.surface.y = y; }
    }

    fn update_sprite_coord(&mut self, dx: f64, dy: f64) {
        if dx == 0. { self.sprite_coord.1 = 0; }
        else if dx > 0. { self.sprite_coord.1 = 1; }
        else { self.sprite_coord.1 = 2; }
        if dy == 0. { self.sprite_coord.0 = 1; }
        else if dy > 0. { self.sprite_coord.0 = 2; }
        else { self.sprite_coord.0 = 0; }
    }
}
