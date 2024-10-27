use text_editor::editor::{Dimensions, Editor};
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let mut editor = Editor::new(Dimensions {
        height: HEIGHT,
        width: WIDTH,
    })?;
    let _ = editor.start();
    Ok(())
}
