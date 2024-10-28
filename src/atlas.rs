use crate::editor::Dimensions;
use sdl2::{pixels::Color, rect::Rect, surface::SurfaceRef};

pub struct Atlas {
    glyphs: Vec<Rect>,
    font_size: Dimensions,
}

const NUM_GLYPHS: usize = 128;
const FONT_TEXTURE_SIZE: u32 = 512;

impl Atlas {
    pub fn new<S>(expected_font_size: u16, surface: &mut S) -> Result<Atlas, String>
    where
        S: AsMut<SurfaceRef>,
    {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("FiraCode-VariableFont_wght.ttf", expected_font_size)?;

        let char_size: (u32, u32) = font
            .size_of_latin1(&[32 as u8])
            .map_err(|e| e.to_string())?;
        let font_size = Dimensions {
            height: char_size.1,
            width: char_size.0,
        };
        // stores all the ASCII char positions
        let mut glyphs: Vec<Rect> = vec![Rect::new(0, 0, 0, 0); NUM_GLYPHS + 1];

        //surface.set_color_key(true, Color::BLACK)?;

        let mut position = Rect::new(0, 0, font_size.width, font_size.height);

        for i in 32u8..127 {
            let char_surface = font
                .render(&(i as char).to_string())
                .blended(Color::RGBA(255, 255, 255, 0))
                .map_err(|e: sdl2::ttf::FontError| e.to_string())?;

            println!("{:?}", position);
            // check to make sure texture fits in the width
            if position.width() + position.x() as u32 >= FONT_TEXTURE_SIZE {
                position.set_x(0);
                position.set_y(position.y() + position.height() as i32 + 1);

                if position.y() as u32 + position.height() >= FONT_TEXTURE_SIZE {
                    panic!("Ran out of atlas space for glyphs")
                }
            }
            char_surface.blit(None, surface.as_mut(), position)?;
            glyphs[usize::from(i)] = position.clone();
            position.set_x(position.x() + position.width() as i32);
        }

        Ok(Atlas { glyphs, font_size })
    }

    pub fn get_char(&self, character: &char) -> Rect {
        return self.glyphs[*character as usize];
    }

    pub fn get_font_size(&self) -> &Dimensions {
        &self.font_size
    }
    // fn create_glyph_cache(font_path: &str) -> Vec<Rect> {
    //     Vec::new()
    // }
}
