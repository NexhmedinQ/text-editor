extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const FONT_SIZE: u16 = 16;
const NUM_GLYPHS: usize = 128;
const FONT_TEXTURE_SIZE: u32 = 512;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Text Editor", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // let masks = pixels::PixelMasks {
    //     rmask: 0xff,
    //     gmask: 0,
    //     bmask: 0,
    //     amask: 0,
    //     bpp: 15,
    // };
    let masks = PixelFormatEnum::RGB24.into_masks().unwrap();
    let mut surface = Surface::from_pixelmasks(512, 512, &masks).unwrap();

    let font = ttf_context.load_font("FiraCode-VariableFont_wght.ttf", FONT_SIZE)?;

    // stores all the ASCII char positions
    let mut glyphs: Vec<Rect> = vec![Rect::new(0, 0, 0, 0); NUM_GLYPHS + 1];

    SurfaceRef::set_color_key(&mut surface, true, Color::BLACK)?;

    let mut position = Rect::new(0, 0, 0, 0);

    for i in 32..127 {
        let i: u8 = i as u8;
        println!("{:?}", i as char);
        let char_surface = font
            .render(&(i as char).to_string())
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e: sdl2::ttf::FontError| e.to_string())?;

        let char_size: (u32, u32) = font.size_of_latin1(&[i as u8]).map_err(|e| e.to_string())?;

        position.set_width(char_size.0);
        position.set_height(char_size.1);
        println!("{:?}", position);
        // check to make sure texture fits in the width
        if position.width() + position.x() as u32 >= FONT_TEXTURE_SIZE {
            position.set_x(0);
            position.set_y(position.y() + position.height() as i32 + 1);

            if position.y() as u32 + position.height() >= FONT_TEXTURE_SIZE {
                panic!("Ran out of atlas space for glyphs")
            }
        }
        char_surface.blit(None, &mut surface, position)?;
        glyphs[i as usize] = position.clone();
        position.set_x(position.x() + position.width() as i32);
    }

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_from_surface(surface).unwrap();

    let test_str = "Let us try to render text onto window";

    texture.set_color_mod(255, 255, 255);

    let mut dst = Rect::new(0, 30, 0, 0);

    for character in test_str.chars() {
        let src_rect = glyphs[character as usize];
        dst.set_width(src_rect.width());
        dst.set_height(src_rect.height());
        canvas.copy(&texture, src_rect, dst).unwrap();
        dst.set_x(dst.x() + src_rect.width() as i32);
    }

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}