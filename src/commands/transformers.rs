use std::{fmt::Debug, io::Write};

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum TransformerException {
    Quit,
}

pub trait Transformer: Debug {
    fn apply(&self, text: &mut String, writer: &mut dyn Write) -> Result<(), TransformerException>;
}

#[derive(Debug)]
pub struct Delete;

impl Transformer for Delete {
    fn apply(
        &self,
        text: &mut String,
        _writer: &mut dyn Write,
    ) -> Result<(), TransformerException> {
        *text = "".to_string();
        Ok(())
    }
}

#[derive(Debug)]
pub struct Substitute {
    find: Regex,
    replace: String,
    global: bool,
}

impl Substitute {
    pub fn new(find: Regex, replace: String, global: bool) -> Self {
        Self {
            find,
            replace,
            global,
        }
    }
}

impl Transformer for Substitute {
    fn apply(
        &self,
        text: &mut String,
        _writer: &mut dyn Write,
    ) -> Result<(), TransformerException> {
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
    fn apply(
        &self,
        _text: &mut String,
        _writer: &mut dyn Write,
    ) -> Result<(), TransformerException> {
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


#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    #[test]
    fn test_delete() {
        let mut text = String::from("Hello world");
        let mut output = Vec::<u8>::new();

        assert_eq!(Delete.apply(&mut text, &mut output), Ok(()));
        assert_eq!(String::from_utf8(output).unwrap(), "");
        assert_eq!(text, "");
    }

    quickcheck! {
        fn test_delete_prop(text: String) -> bool {
            let mut output = Vec::<u8>::new();
            let mut textcopy = text.clone();
            let _ = Delete.apply(&mut textcopy, &mut output);
            textcopy == ""
        }
    }

    #[test]
    fn test_print() {
        let mut text = String::from("Hello world");
        let mut output = Vec::<u8>::new();

        assert_eq!(Print.apply(&mut text, &mut output), Ok(()));
        assert_eq!(String::from_utf8(output).unwrap(), text);
        assert_eq!(text, "Hello world");
    }

    quickcheck! {
        fn test_print_prop(text: String) -> bool {
            let mut output = Vec::<u8>::new();
            let mut textcopy = text.clone();
            let _ = Print.apply(&mut textcopy, &mut output);
            textcopy == text && String::from_utf8(output).unwrap() == text
        }
    }

    #[test]
    fn test_quit() {
        let mut text = String::from("Hello world");
        let mut output = Vec::<u8>::new();

        assert_eq!(Quit.apply(&mut text, &mut output), Err(TransformerException::Quit));
        assert_eq!(String::from_utf8(output).unwrap(), "");
        assert_eq!(text, "Hello world");
    }

    #[test]
    fn test_substitute_basic() {
        let mut text = String::from("Hello World, Hello World");
        let mut output = Vec::<u8>::new();
        let substitute = Substitute::new(
            Regex::new("Hello").unwrap(),
            "World".to_string(),
            false
        );

        assert_eq!(substitute.apply(&mut text, &mut output), Ok(()));
        assert_eq!(String::from_utf8(output).unwrap(), "");
        assert_eq!(text, "World World, Hello World");
    }

    #[test]
    fn test_substitute_basic_global() {
        let mut text = String::from("Hello World, Hello World");
        let mut output = Vec::<u8>::new();
        let substitute = Substitute::new(
            Regex::new("Hello").unwrap(),
            "World".to_string(),
            true
        );

        assert_eq!(substitute.apply(&mut text, &mut output), Ok(()));
        assert_eq!(String::from_utf8(output).unwrap(), "");
        assert_eq!(text, "World World, World World");
    }

    #[test]
    fn test_substitute_no_match() {
        let mut text = String::from("Hello");
        let mut output = Vec::<u8>::new();
        let substitute = Substitute::new(
            Regex::new("Goodbye").unwrap(),
            "Planet".to_string(),
            false
        );

        assert_eq!(substitute.apply(&mut text, &mut output), Ok(()));
        assert_eq!(String::from_utf8(output).unwrap(), "");
        assert_eq!(text, "Hello");
    }
}