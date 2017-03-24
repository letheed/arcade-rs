use config::Config;
use sdl::{EventPump, Sdl};
use sdl::render::Renderer;
use sdl::image::Sdl2ImageContext as SdlImg;

pub struct SdlContext {
    sdl: Sdl,
    _sdl_img: SdlImg,
}

impl SdlContext {
    pub fn init<'w>(config: &Config) -> (Self, EventPump, Renderer<'w>) {
        let sdl = ::sdl::init().unwrap();
        let sdl_img = ::sdl::image::init(::sdl::image::INIT_PNG).unwrap();
        let context = SdlContext::new(sdl, sdl_img);
        let event_pump = context.sdl.event_pump().unwrap();
        let renderer = context.renderer(config);
        (context, event_pump, renderer)
    }
}

impl SdlContext {
    fn new(sdl: Sdl, sdl_img: SdlImg) -> Self {
        SdlContext {
            sdl: sdl,
            _sdl_img: sdl_img,
        }
    }

    fn renderer<'w>(&self, config: &Config) -> Renderer<'w> {
        let (width, height) = config.definition;
        let video = self.sdl.video().unwrap();
        let window = video.window(config.name, width, height)
                          .resizable()
                          .position_centered()
                          .opengl()
                          .build().unwrap();
        let renderer = window.renderer()
                             .accelerated()
                             .build().unwrap();
        renderer
    }
}
