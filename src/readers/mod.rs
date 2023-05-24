pub mod file_reader;
pub mod reader;
pub mod stdin_reader;
pub mod general_reader;

pub use file_reader::FileReader;
pub use reader::{Line, Reader};
pub use stdin_reader::StdinReader;
pub use general_reader::GeneralReader;
