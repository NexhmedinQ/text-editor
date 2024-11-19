use sdl2::{
    keyboard::Keycode,
    pixels::{self, Color},
    rect::Rect,
    render::Canvas,
    surface::SurfaceRef,
    video::Window,
    Sdl,
};

use crate::{atlas::Atlas, editor::Dimensions, text_buffer::Buffer};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Position {
    x: u32,
    y: u32,
}

struct Cursor {
    cursor_pos: Position,
    old_cursor_pos: Position,
    line_num: u32,
}

pub struct Screen {
    cursor: Cursor,
    canvas: Canvas<Window>,
    window_size: Dimensions,
    top_line: u32,
    line_buf: Box<Vec<String>>,
}

impl Screen {
    pub fn new(
        sdl_context: &Sdl,
        dimensions: &Dimensions,
        text_buffer: &Buffer,
    ) -> Result<Screen, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Text Editor", dimensions.width, dimensions.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        Ok(Screen {
            cursor: Cursor {
                cursor_pos: Position { x: 0, y: 0 },
                old_cursor_pos: Position { x: 0, y: 0 },
                line_num: 0,
            },
            canvas,
            top_line: 0,
            window_size: Dimensions {
                height: dimensions.height,
                width: dimensions.width,
            },
            line_buf: Box::new(text_buffer.get_lines(0, 1)),
        })
    }

    pub fn draw_text<S>(&mut self, text_buffer: &mut Buffer, surface: S, atlas: &Atlas)
    where
        S: AsRef<SurfaceRef>,
    {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();

        texture.set_color_mod(255, 255, 255);

        let mut dst = Rect::new(0, self.top_line as i32, 0, 0);

        let char_size = atlas.get_font_size();
        let chars_wide = self.window_size.width.div_ceil(char_size.width);
        let num_lines = self.window_size.height.div_ceil(char_size.height);

        let line_buf = text_buffer.get_lines(self.top_line, self.top_line + num_lines);
        self.line_buf = Box::new(line_buf);
        for line in &*self.line_buf {
            dst.set_x(0);
            for (index, character) in line.as_bytes().iter().enumerate() {
                if index > chars_wide.try_into().unwrap() {
                    break;
                }
                let src_rect = atlas.get_char(&(*character as char));
                dst.set_width(src_rect.width());
                dst.set_height(src_rect.height());
                self.canvas.copy(&texture, src_rect, dst).unwrap();
                dst.set_x(dst.x() + i32::try_from(src_rect.width()).unwrap());
            }
            dst.set_y(dst.y() + i32::try_from(char_size.height).unwrap());
        }
    }

    pub fn draw_cursor(&mut self, atlas: &Atlas) {
        let font_size = atlas.get_font_size();
        let coordinate_cursor_x = self.cursor.cursor_pos.x * font_size.width;
        let coordinate_cursor_y = self.cursor.cursor_pos.y * font_size.height;
        let mut width = 2;
        if font_size.width >= 10 {
            width = (font_size.width as f64 * 0.2).floor() as i32;
            if width % 2 != 0 {
                width -= 1;
            }
        }
        self.canvas.set_draw_color(pixels::Color {
            r: 255,
            g: 255,
            b: 255,
            a: 0,
        });
        self.canvas
            .fill_rect(Rect::new(
                coordinate_cursor_x as i32 - (width / 2),
                coordinate_cursor_y as i32,
                width as u32,
                font_size.height,
            ))
            .unwrap();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn clear_screen(&mut self) {
        self.canvas.clear();
    }

    pub fn colour(&mut self, colour: Color) {
        self.canvas.set_draw_color(colour);
    }

    pub fn cursor_move(&mut self, direction: Keycode) {
        match direction {
            Keycode::LEFT => {
                if self.cursor.cursor_pos.x != 0 {
                    self.cursor.cursor_pos.x -= 1;
                } else if self.cursor.cursor_pos.y > 0 {
                    self.cursor.cursor_pos.y -= 1;
                    self.cursor.cursor_pos.x =
                        self.line_buf[self.cursor.cursor_pos.y as usize].len() as u32;
                    self.cursor.line_num -= 1;
                }
                self.cursor.old_cursor_pos = self.cursor.cursor_pos.clone();
            }
            Keycode::RIGHT => {
                if self.cursor.cursor_pos.x
                    < self.line_buf[self.cursor.cursor_pos.y as usize].len() as u32
                {
                    self.cursor.cursor_pos.x += 1;
                } else if self.cursor.cursor_pos.y < (self.line_buf.len() - 1) as u32 {
                    self.cursor.cursor_pos.y += 1;
                    self.cursor.cursor_pos.x = 0;
                    self.cursor.line_num += 1;
                }
                self.cursor.old_cursor_pos = self.cursor.cursor_pos.clone();
            }
            Keycode::DOWN => {
                if self.cursor.cursor_pos.y < (self.line_buf.len() - 1) as u32 {
                    self.cursor.cursor_pos.y += 1;
                    Self::set_cursor_x_on_vert_move(self);
                    self.cursor.line_num += 1;
                }
            }
            Keycode::UP => {
                if self.cursor.cursor_pos.y > 0 {
                    self.cursor.cursor_pos.y -= 1;
                    Self::set_cursor_x_on_vert_move(self);
                    self.cursor.line_num -= 1;
                }
            }
            _ => unreachable!("method is only called for when keycode is a direction"),
        }
    }

    fn set_cursor_x_on_vert_move(&mut self) {
        if self.line_buf[self.cursor.cursor_pos.y as usize].len()
            >= self.cursor.old_cursor_pos.x as usize
        {
            self.cursor.cursor_pos.x = self.cursor.old_cursor_pos.x;
        } else {
            self.cursor.cursor_pos.x =
                self.line_buf[self.cursor.cursor_pos.y as usize].len() as u32;
        }
    }

    pub fn get_cursor_line_position(&self) -> (u32, u32) {
        (self.cursor.line_num, self.cursor.cursor_pos.y)
    }
}
