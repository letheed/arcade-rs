use sdl::rect::Rect as SdlRect;
use sdl::render::{Renderer, Texture, TextureQuery};
use sdl::image::LoadTexture;
use std::path::Path;

pub struct Sprite {
    texture: Texture,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn new(renderer: &Renderer, texture_path: &str) -> Self {
        let texture = renderer.load_texture(Path::new(texture_path)).unwrap();
        let TextureQuery { width: tex_width, height: tex_height, .. } = texture.query();
        Sprite {
            texture: texture,
            width: tex_width,
            height: tex_height,
        }
    }

    pub fn render(&self, renderer: &mut Renderer, surface: SdlRect) {
        renderer.copy(&self.texture, None, Some(surface)).unwrap();
    }
}

pub struct SpriteSheet {
    texture: Texture,
    rows: u32,
    cols: u32,
    pub width: u32,
    pub height: u32,
}

impl SpriteSheet {
    pub fn new(renderer: &Renderer, texture_path: &str, (rows, cols): (u32, u32)) -> Self {
        let texture = renderer.load_texture(Path::new(texture_path)).unwrap();
        let TextureQuery { width: tex_width, height: tex_height, .. } = texture.query();
        debug_assert!(tex_width % cols == 0 && tex_height % rows == 0);
        SpriteSheet {
            texture: texture,
            rows: rows,
            cols: cols,
            width: tex_width / cols,
            height: tex_height / rows,
        }
    }

    pub fn render(&self, renderer: &mut Renderer, sprite_coord: (u32, u32), surface: SdlRect) {
        renderer.copy(&self.texture, Some(self.select(sprite_coord)), Some(surface)).unwrap();
    }
}

impl SpriteSheet {
    fn select(&self, (row, col): (u32, u32)) -> SdlRect {
        debug_assert!(row < self.rows && col < self.cols);
        SdlRect::new((col * self.width) as i32, (row * self.height) as i32, self.width, self.height)
    }
}
