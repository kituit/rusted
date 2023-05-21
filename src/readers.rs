use std::{io::{stdin, BufReader, BufRead, Lines}, fs::File, path::Path};

pub struct StdinReader {
    curr_line: u64,
}

impl StdinReader {
    pub fn new() -> Self {
        Self { curr_line: 0 }
    }
}

impl Iterator for StdinReader {
    type Item = (u64, String);

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

// struct FileReader {
//     curr_line: u64,
//     files: Vec<String>,
//     curr_file: Option<Lines<BufReader<File>>>
// }

// impl FileReader {
//     fn new(files: Vec<String>) -> Self {
//         let files = files.into_iter()
//             .map(|file_name| (file_name, Path::new(&file_name)))
//             .filter(|(file_name, file_path)| {
//                 let exists = file_path.exists();
//                 if !exists { eprintln!("ERROR: File {file_name} does not exist") }
//                 exists
//             })
//             .map(|(file_name, _)| file_name)
//             .collect::<Vec<String>>();

//         let curr_file = files.get(0).and_then(|file_name| Some(File::open(file_name).unwrap()));

//         Self {
//             curr_line: 0,
//             files,
//             curr_file
//         }
//     }
// }

// impl Iterator for FileReader {
//     type Item = (u64, String);

//     fn next(&mut self) -> Option<Self::Item> {
//         // match &self.curr_file {
//         //     Some(file) => {
                
//         //     },
//         //     None => None
//         // }

//         if let Some(file) = &self.curr_file {
//             let next_line = file.next().and_then(|line| Some(line.expect("File error")));

//             if next_line.is_some() {
//                 return next_line;
//             }


//         }



//         todo!()
//     }
// }
