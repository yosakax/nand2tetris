mod code_writer;
mod parser;

fn main() {
    let mut parser = parser::Parser::new("src/main.rs").unwrap();
    let mut code_writer = code_writer::CodeWriter::new("tmp.txt").unwrap();
    while parser.has_more_lines().unwrap() {
        parser.advance();
        // なんかやる
        let line = parser.next_line();
        // eprintln!("{}", line);
        code_writer.write_arithmetic(line.as_str());
    }
}

