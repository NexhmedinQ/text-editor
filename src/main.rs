use std::env;

use text_editor::editor::{Dimensions, Editor};
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run <filepath>")
    }
    let mut editor = Editor::new(Dimensions {
        height: HEIGHT,
        width: WIDTH,
    },
args[1].clone())?;
    let _ = editor.start();
    Ok(())
}
