use sdl2::{event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, render::Canvas, surface::Surface, video::Window, Sdl};

use crate::{atlas::Atlas, screen::Screen};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32
}

pub struct Editor<'a> {
    atlas: Atlas,
    screen: Screen,
    sdl_context: Sdl,
    surface: Surface<'a>,
}

impl<'a> Editor<'a> {

    pub fn new(dimensions: Dimensions) -> Result<Editor<'a>, String> {
        let sdl_context = sdl2::init()?;

        let masks = PixelFormatEnum::RGB24.into_masks().unwrap();
        let mut surface = Surface::from_pixelmasks(512, 512, &masks).unwrap();
        surface.set_color_key(true, Color::BLACK)?;
        let screen = Screen::new(&sdl_context, &dimensions)?;
        return Ok(Editor { sdl_context, atlas: Atlas::new(16, &mut surface)?, screen, surface });
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.screen.draw_text("Let us try to render text onto window", self.surface.as_ref(), &self.atlas);
        self.screen.render();
        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {

            let ctrl_pressed = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .any(|key| key == Keycode::LCTRL || key == Keycode::RCTRL);

            for event in event_pump.poll_iter() {
                if ctrl_pressed {
                    if let Event::KeyDown { keycode,.. } = event {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::S => println!("Save file"),
                                Keycode::LEFTBRACKET => println!("redo last operation in history"),
                                Keycode::RIGHTBRACKET => println!("undo last operation in history"),
                                _ => println!("World")
                            }
                        }
                    }
                } else {
                    match event {
                        Event::Quit { .. } => break 'running,
                        Event::KeyDown {keycode, ..} => {
                            if let Some(key) = keycode {
                                match key {
                                    Keycode::BACKSPACE => println!("Backspace"),
                                    Keycode::TAB => println!("TAB"),
                                    Keycode::RETURN => println!("Return"),
                                    Keycode::UP | Keycode::DOWN | Keycode::LEFT | Keycode::RIGHT => println!("cursor position"),
                                    _ => println!("Other keycode")
                                }
                            }
                        },
                        Event::TextInput {text, ..} => println!("{}", text),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    
}