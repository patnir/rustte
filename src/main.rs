mod editor;
use editor::Editor;
mod terminal;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
