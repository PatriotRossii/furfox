#![warn(clippy::all, clippy::pedantic)]

use editor::Editor;

pub mod editor;

fn main() {
    Editor::default().run();
}
