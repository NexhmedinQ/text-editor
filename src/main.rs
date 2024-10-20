extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use text_editor::editor::{Dimensions, Editor};
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FONT_SIZE: u16 = 16;
const NUM_GLYPHS: usize = 128;
const FONT_TEXTURE_SIZE: u32 = 512;

pub fn main() -> Result<(), String> {
    let mut editor = Editor::new(Dimensions { height: HEIGHT, width: WIDTH })?;
    let _ = editor.start();
    Ok(())
}
