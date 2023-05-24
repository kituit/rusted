use std::{iter::Enumerate, io::{BufReader, BufRead, Lines}, fs::File, path::Path};

pub struct FileReader {
    files: Vec<String>,
    curr_file: Option<Enumerate<Lines<BufReader<File>>>>
}

impl FileReader {
    pub fn new(mut files: Vec<String>) -> Self {
        for file_name in files.iter() {
            let file_path = Path::new(file_name);
            if !file_path.is_file() {
                panic!("ERROR: No such file {file_name}");
            }
        }

        files.reverse();

        Self {
            files,
            curr_file: None
        }
    }

    fn open_next_file(&mut self) {
        if let Some(file_name) = self.files.pop() {
            self.curr_file = Some(BufReader::new(File::open(file_name).unwrap()).lines().enumerate());
        }
    }

}

impl Iterator for FileReader {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
       
        if self.curr_file.is_none() {
            self.open_next_file();
            if self.curr_file.is_none() { return None }
        }

        let curr_file = &mut self.curr_file.as_mut().unwrap();

        if let Some((line_number, line)) = curr_file.next() {
            let line_number = line_number + 1;
            let mut line = line.expect("Error reading from file");
            line.push('\n');
            Some((line_number, line))
        } else {
            self.curr_file = None;
            self.next()
        }
    }


}
