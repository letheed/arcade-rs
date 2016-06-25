use sdl::render::Renderer;
use self::sprite::SpriteSheet;

pub mod sprite;

pub struct Assets {
    pub ship: SpriteSheet,
}

impl Assets {
    pub fn load(renderer: &Renderer) -> Self {
        Assets {
            ship: SpriteSheet::new(renderer, &full_path!("assets/spaceship.png"), (3, 3)),
        }
    }
}
