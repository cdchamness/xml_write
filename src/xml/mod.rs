use std::io::Write;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Indentation {
    indent_char: char,
    size: u8,
    count: u8,
}

impl Default for Indentation {
    fn default() -> Self {
        Self {
            indent_char: ' ',
            size: 2,
            count: 0,
        }
    }
}

impl Indentation {
    pub fn new(indent_char: char, size: u8) -> Indentation {
        Indentation {
            indent_char,
            size,
            count: 0,
        }
    }

    pub fn get_indent(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.count {
            out += &(0..self.size).map(|_| self.indent_char).collect::<String>();
        }
        out
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) -> Option<bool> {
        if self.count < 1 {
            return None;
        }
        self.count -= 1;
        Some(true)
    }
}

#[derive(Debug)]
pub struct XMLWriter<T: Write> {
    indentation: Indentation,
    tag_buffer: Vec<String>,
    writer: T,
}

impl<T: Write> XMLWriter<T> {
    pub fn new_with_indent(indentation: Indentation, writer: T) -> XMLWriter<T> {
        XMLWriter {
            indentation,
            tag_buffer: vec![],
            writer,
        }
    }

    pub fn new(mut writer: T) -> Result<XMLWriter<T>, Error> {
        writer.write_all("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>".as_bytes())?;
        Ok(XMLWriter {
            indentation: Indentation::default(),
            tag_buffer: vec![],
            writer,
        })
    }

    pub fn open_tag(mut self, tagname: &str) -> Result<Self, Error> {
        self.writer
            .write_all(format!("{}<{}>\n", self.indentation.get_indent(), tagname).as_bytes())?;
        self.indentation.increment();
        self.tag_buffer.push(tagname.to_string());
        Ok(self)
    }

    pub fn close_tag(mut self) -> Result<Self, Error> {
        self.indentation
            .decrement()
            .ok_or_else(|| Error::new(ErrorKind::Other, "No open tag to close!"))?;
        self.writer.write_all(
            format!(
                "{}</{}>\n",
                self.indentation.get_indent(),
                self.tag_buffer.pop().unwrap()
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    pub fn add_elem(mut self, tagname: &str, val: &str) -> Result<Self, Error> {
        self.writer.write_all(
            format!(
                "{}<{}>{}</{}>\n",
                self.indentation.get_indent(),
                tagname,
                val,
                tagname
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    pub fn flush(mut self) -> Result<Self, Error> {
        self.writer.flush()?;
        Ok(self)
    }
}
