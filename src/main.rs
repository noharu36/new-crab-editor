pub use crab::document::Document;
use crab::editor::Editor;
pub use crab::editor::Position;
pub use crab::row::Row;
pub use crab::terminal::Terminal;

fn main() {
    Editor::default().run();
}
