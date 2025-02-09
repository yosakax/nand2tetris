use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    pub stream: BufWriter<File>,
    current_pos: usize,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let file = File::create(file_path)?;
        let stream = BufWriter::new(file);
        return Ok(CodeWriter {
            stream,
            current_pos: 0,
        });
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        // writeln!(self.stream, command.to_string(), self.current_pos).unwrap();
        self.stream.write_all(command.as_bytes()).unwrap();
        self.stream.write_all(b"\n").unwrap();
        self.current_pos += 1;
        self.stream.flush().unwrap();
    }
}

