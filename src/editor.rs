use std::{
    path::Path,
    time::{Duration, Instant},
};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{self, Color, PixelFormatEnum},
    surface::Surface,
    Sdl,
};

use crate::{atlas::Atlas, screen::Screen, text_buffer::{self, Buffer}};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

pub struct Editor<'a> {
    atlas: Atlas,
    screen: Screen,
    sdl_context: Sdl,
    surface: Surface<'a>,
    text_buffer: Buffer,
}

#[derive(PartialEq)]
enum CursorState {
    On,
    Off,
}

impl<'a> Editor<'a> {
    pub fn new<P: AsRef<Path>>(dimensions: Dimensions, file_path: P) -> Result<Editor<'a>, String> {
        let sdl_context = sdl2::init()?;

        let masks = PixelFormatEnum::RGB24.into_masks().unwrap();
        let mut surface = Surface::from_pixelmasks(512, 512, &masks).unwrap();
        surface.set_color_key(true, Color::BLACK)?;
        let text_buffer = Buffer::open(file_path).unwrap();
        let screen = Screen::new(&sdl_context, &dimensions, &text_buffer)?;
        return Ok(Editor {
            sdl_context,
            atlas: Atlas::new(16, &mut surface)?,
            screen,
            surface,
            text_buffer
        });
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut cursor_state = CursorState::On;
        let mut time_since_cursor_change = Instant::now();

        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            self.screen.colour(pixels::Color { r: 0, g: 0, b: 0, a: 0 });
            self.screen.clear_screen();
            Self::manage_cursor(&mut time_since_cursor_change, &mut cursor_state, false);
            self.screen
                .draw_text(&mut self.text_buffer, self.surface.as_ref(), &self.atlas);

            let ctrl_pressed = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .any(|key| key == Keycode::LCTRL || key == Keycode::RCTRL);

            for event in event_pump.poll_iter() {
                if ctrl_pressed {
                    if let Event::KeyDown { keycode, .. } = event {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::S => println!("Save file"),
                                Keycode::LEFTBRACKET => println!("redo last operation in history"),
                                Keycode::RIGHTBRACKET => println!("undo last operation in history"),
                                _ => println!("World"),
                            }
                        }
                    }
                } else {
                    match event {
                        Event::Quit { .. } => break 'running,
                        Event::KeyDown { keycode, .. } => {
                            if let Some(key) = keycode {
                                match key {
                                    Keycode::BACKSPACE => println!("Backspace"),
                                    Keycode::TAB => println!("TAB"),
                                    Keycode::RETURN => println!("Return"),
                                    Keycode::UP
                                    | Keycode::DOWN
                                    | Keycode::LEFT
                                    | Keycode::RIGHT => {
                                        self.screen.cursor_move(key);
                                        Self::manage_cursor(
                                            &mut time_since_cursor_change,
                                            &mut cursor_state,
                                            true,
                                        );
                                    }
                                    _ => println!("Other keycode"),
                                }
                            }
                        }
                        Event::TextInput { text, .. } => println!("{}", text),
                        _ => {}
                    }
                }
            }
            if cursor_state == CursorState::On {
                self.screen.draw_cursor(&self.atlas);
            }
            self.screen.render();
        }
        Ok(())
    }

    fn manage_cursor(
        time_since_state_change: &mut Instant,
        cursor_state: &mut CursorState,
        refresh_on_state: bool,
    ) {
        if refresh_on_state {
            *time_since_state_change = Instant::now();
            *cursor_state = CursorState::On;
        }
        let now = Instant::now();
        if now
            .checked_duration_since(*time_since_state_change)
            .unwrap()
            >= Duration::from_millis(600)
            && *cursor_state == CursorState::Off
        {
            //turn on
            *cursor_state = CursorState::On;
            *time_since_state_change = Instant::now();
        } else if now
            .checked_duration_since(*time_since_state_change)
            .unwrap()
            >= Duration::from_millis(400)
            && *cursor_state == CursorState::On
        {
            // turn off
            *cursor_state = CursorState::Off;
            *time_since_state_change = Instant::now();
        }
    }
}
