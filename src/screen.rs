use sdl2::{pixels::Color, rect::Rect, render::Canvas, surface::{self, SurfaceRef}, video::Window, Sdl};

use crate::{atlas::Atlas, editor::Dimensions};

#[derive(Debug, PartialEq, PartialOrd)]
struct Position {
    x: u32,
    y: u32,
}
pub struct Screen {
    cursor_pos: Position,
    canvas: Canvas<Window>,
    window_size: Dimensions,
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
       Ok(Screen { cursor_pos: Position { x: 0, y: 0 }, canvas, window_size: Dimensions { height: dimensions.height, width: dimensions.width } })
    }

    pub fn draw_text<S>(&mut self, text: &str, surface: S, atlas: &Atlas) where S: AsRef<SurfaceRef> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();

        // let test_str = "Let us try to render text onto window";

        texture.set_color_mod(255, 255, 255);

        let mut dst = Rect::new(0, 0, 0, 0);

        for character in text.chars() {
            let src_rect = atlas.get_char(&character);
            dst.set_width(src_rect.width());
            dst.set_height(src_rect.height());
            self.canvas.copy(&texture, src_rect, dst).unwrap();
            dst.set_x(dst.x() + i32::try_from(src_rect.width()).unwrap());
        }
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
