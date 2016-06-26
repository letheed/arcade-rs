use sdl::EventPump;
use sdl::event::Event::{KeyDown as SdlKeyDown, KeyUp as SdlKeyUp, Window, Quit};
use sdl::event::WindowEventId;
use sdl::keyboard::Keycode::{Escape, Space, Up, Down, Left, Right};
use super::State;
use self::KeyEvent::*;

#[derive(Eq, PartialEq)]
pub enum KeyEvent {
    KeyUp,
    KeyDown,
    NoKeyEvent,
}

macro_rules! impl_input {
    ( keys { $($key_name:ident: $key_code:ident,)*     },
      signals  { $($event_name:ident: $event_code:ident,)* },
    ) => {
        pub struct InputState {
            $(pub $key_name: bool,)*
        }

        impl InputState {
            pub fn new() -> Self {
                InputState {
                    $($key_name: false,)*
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            $(pub $key_name: KeyEvent,)*
            $(pub $event_name: bool,)*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Self {
                Events {
                    pump: pump,
                    $($key_name: NoKeyEvent,)*
                    $($event_name: false,)*
                }
            }

            fn clear(&mut self) {
                $(self.$key_name = NoKeyEvent;)*
                $(self.$event_name = false;)*
            }
        }

        impl State {
            pub fn pump_events(&mut self) {
                self.events.clear();
                for event in self.events.pump.poll_iter() {
                    match event {
                        Window { win_event_id, data1, data2, .. } => match win_event_id {
                            WindowEventId::Resized => {
                                debug_assert!(data1 >= 0 && data2 >= 0);
                                self.definition = (data1 as u32, data2 as u32);
                                self.definitionf64 = (data1 as f64, data2 as f64);
                            },
                            _ => {},
                        },
                        SdlKeyDown { keycode, .. } => if let Some(keycode) = keycode {
                            match keycode {
                                $($key_code => {
                                    if !self.input.$key_name {
                                        self.input.$key_name = true;
                                        self.events.$key_name = KeyDown;
                                    }
                                },)*
                                _ => {},
                            }
                        },
                        SdlKeyUp { keycode, .. } => if let Some(keycode) = keycode {
                            match keycode {
                                $($key_code => {
                                    self.input.$key_name = false;
                                    self.events.$key_name = KeyUp;
                                },)*
                                _ => {},
                            }
                        },
                        $($event_code { .. } => self.events.$event_name = true,)*
                        _ => {},
                    }
                }
            }
        }
    }
}

impl_input! {
    keys {
        key_escape: Escape,
        key_space: Space,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
    },
    signals {
        quit: Quit,
    },
}
