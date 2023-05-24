pub mod reader;
pub mod stdin_reader;
pub mod file_reader;

pub use reader::{Reader, Line};
pub use stdin_reader::StdinReader;
pub use file_reader::FileReader;
