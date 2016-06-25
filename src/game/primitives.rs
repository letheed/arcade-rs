use sdl::rect::Rect as SdlRect;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        debug_assert!(x >= 0. && y >= 0. && w > 0. && h > 0.);
        Rectangle { x: x, y: y, w: w, h: h }
    }

    pub fn to_sdl(&self) -> SdlRect {
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}
