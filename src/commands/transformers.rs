use std::{fmt::Debug, io::Write};

use regex::Regex;

#[derive(Debug)]
pub enum TransformerException {
    Quit,
}

pub trait Transformer: Debug {
    fn apply(&self, text: &mut String, writer: &mut dyn Write) -> Result<(), TransformerException>;
}

#[derive(Debug)]
pub struct Delete;

impl Transformer for Delete {
    fn apply(&self, text: &mut String, _writer: &mut dyn Write) -> Result<(), TransformerException> {
        *text = "".to_string();
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
    fn apply(&self, text: &mut String, _writer: &mut dyn Write) -> Result<(), TransformerException> {
        if self.global {
            *text = self.find.replace_all(text, &self.replace).to_string();
        } else {
            *text = self.find.replace(text, &self.replace).to_string();
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct Quit;

impl Transformer for Quit {
    fn apply(&self, _text: &mut String, _writer: &mut dyn Write) -> Result<(), TransformerException> {
        Err(TransformerException::Quit)
    }
}


#[derive(Debug)]
pub struct Print;

impl Transformer for Print {
    fn apply(&self, text: &mut String, writer: &mut dyn Write) -> Result<(), TransformerException> {
        let _ = write!(writer, "{text}");
        Ok(())
    }
}