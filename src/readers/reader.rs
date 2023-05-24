pub struct Line {
    pub line_number: usize,
    pub text: String,
    pub is_last_line: bool,
}

enum Buffer {
    FirstLine,
    Continue(Option<(usize, String)>),
}

impl Buffer {
    fn get_inner(&self) -> Option<&(usize, String)> {
        match self {
            Buffer::FirstLine => None,
            Buffer::Continue(buffer) => buffer.as_ref(),
        }
    }

    fn into_inner(self) -> Option<(usize, String)> {
        match self {
            Buffer::FirstLine => None,
            Buffer::Continue(inner) => inner,
        }
    }
}

pub struct Reader<E> {
    iter: E,
    buffer: Buffer,
}

impl<E> Reader<E> {
    pub fn new(iter: E) -> Self {
        Self {
            iter,
            buffer: Buffer::FirstLine,
        }
    }
}

impl<E: Iterator<Item = (usize, String)>> Iterator for Reader<E> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if let Buffer::FirstLine = self.buffer {
            self.buffer = Buffer::Continue(self.iter.next());
        }

        let _ = self.buffer.get_inner()?;

        let next_line = self.iter.next();
        let is_last_line = next_line.is_none();
        let curr_line = std::mem::replace(&mut self.buffer, Buffer::Continue(next_line));
        let (line_number, text) = curr_line.into_inner()?;

        Some(Line {
            line_number,
            text,
            is_last_line,
        })
    }
}
