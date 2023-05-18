use std::fmt::Debug;

use regex::Regex;

#[derive(Debug)]
pub enum TransformerException {
    Quit,
}

pub trait Transformer: Debug {
    fn apply(&self, line: &mut String) -> Result<(), TransformerException>;
}

#[derive(Debug)]
pub struct Delete;

impl Transformer for Delete {
    fn apply(&self, line: &mut String) -> Result<(), TransformerException> {
        *line = "".to_string();
        Ok(())
    }
}


#[derive(Debug)]
pub struct Substitute {
    find: Regex,
    replace: String,
    global: bool
}

impl Substitute {
    pub fn new(find: Regex, replace: String, global: bool) -> Self {
        Self { find, replace, global }
    }
}

impl Transformer for Substitute {
    fn apply(&self, line: &mut String) -> Result<(), TransformerException> {
        if self.global {
            *line = self.find.replace_all(line, &self.replace).to_string();
        } else {
            *line = self.find.replace(line, &self.replace).to_string();
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct Quit;

impl Transformer for Quit {
    fn apply(&self, _line: &mut String) -> Result<(), TransformerException> {
        Err(TransformerException::Quit)
    }
}


#[derive(Debug)]
pub struct Print;

impl Transformer for Print {
    fn apply(&self, line: &mut String) -> Result<(), TransformerException> {
        print!("{line}");
        Ok(())
    }
}