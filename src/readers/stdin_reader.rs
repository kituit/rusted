use std::io::stdin;

#[derive(Default)]
pub struct StdinReader {
    curr_line: usize,
}

impl Iterator for StdinReader {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        // Error reading from input
        if stdin().read_line(&mut buffer).is_err() {
            return None;
        }

        // EOF
        if buffer.is_empty() {
            return None;
        }

        self.curr_line += 1;
        Some((self.curr_line, buffer))
    }
}
