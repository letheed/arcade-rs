use sdl::render::Renderer;
use self::sprite::{Sprite, SpriteSheet};

pub mod sprite;

pub struct Assets {
    pub spaceship: SpriteSheet,
    pub star_bg: Sprite,
    // pub star_mg: Sprite,
    pub star_fg: Sprite,
}

impl Assets {
    pub fn load(renderer: &Renderer) -> Self {
        Assets {
            spaceship: SpriteSheet::new(renderer, &full_path!("assets/spaceship.png"), (3, 3)),
            star_bg: Sprite::new(renderer, &full_path!("assets/starBG.png")),
            // star_mg: Sprite::new(renderer, &full_path!("assets/starMG.png")),
            star_fg: Sprite::new(renderer, &full_path!("assets/starFG.png")),
          }
    }
}
