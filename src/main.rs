use editor::Editor;

pub mod editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
