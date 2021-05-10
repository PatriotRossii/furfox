#![warn(clippy::all, clippy::pedantic)]

use editor::Editor;

pub mod editor;
pub mod terminal;

fn main() {
    Editor::default().run();
}
