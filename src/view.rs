use assets::Assets;
use game::Game;
use sdl::render::Renderer;
use state::State;
use self::ViewType::*;

pub enum ViewAction {
    Present,
    Quit,
}

enum ViewType {
    Menu,
    GameView,
}

pub struct View<'a> {
    view_type: ViewType,
    game: Game<'a>,
}

impl<'a> View<'a> {
    pub fn init(assets: &'a Assets) -> Self {
        View {
            view_type: GameView,
            game: Game::new(assets),
        }
    }

    pub fn render(&mut self, state: &State, renderer: &mut Renderer) -> ViewAction {
        use sdl::pixels::Color;
        use state::input::KeyEvent::*;

        let events = &state.events;
        let input = &state.input;
        if events.quit || input.key_escape {
            return ViewAction::Quit
        }
        match self.view_type {
            Menu => if events.key_space == KeyDown { self.view_type = GameView; },
            GameView => {
                renderer.set_draw_color(Color::RGB(0, 0, 0));
                renderer.clear();
                if events.key_space == KeyDown { self.view_type = Menu; }
                else { self.game.render(state, renderer); }
            },
        }
        ViewAction::Present
    }
}
