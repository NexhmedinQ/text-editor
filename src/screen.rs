use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    surface::SurfaceRef,
    video::Window,
    Sdl,
};

use crate::{atlas::Atlas, editor::Dimensions, text_buffer::Buffer};

#[derive(Debug, PartialEq, PartialOrd)]
struct Position {
    x: u32,
    y: u32,
}
pub struct Screen {
    cursor_pos: Position,
    old_cursor_pos: Position,
    canvas: Canvas<Window>,
    window_size: Dimensions,
    top_line: u32
}

impl Screen {
    pub fn new(sdl_context: &Sdl, dimensions: &Dimensions) -> Result<Screen, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Text Editor", dimensions.width, dimensions.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        Ok(Screen {
            cursor_pos: Position { x: 0, y: 0 },
            old_cursor_pos: Position { x: 0, y: 0 },
            canvas,
            top_line: 0,
            window_size: Dimensions {
                height: dimensions.height,
                width: dimensions.width,
            },
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
        for line in line_buf {
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

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
