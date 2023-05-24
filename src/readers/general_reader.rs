pub struct GeneralReader<I> {
    iter: I,
    curr_line: usize,
}

impl<I> GeneralReader<I> {
    pub fn new(iter: I) -> Self {
        Self { iter, curr_line: 0 }
    }
}

impl<E: ToString, I: Iterator<Item = E>> Iterator for GeneralReader<I> {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.iter.next()?.to_string();
        if !line.ends_with('\n') {
            line.push('\n');
        }

        self.curr_line += 1;
        Some((self.curr_line, line))
    }
}
