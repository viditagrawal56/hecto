use std::{ fs::read_to_string, io::Error };
use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in file_contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { lines })
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
