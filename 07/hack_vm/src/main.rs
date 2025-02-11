mod code_writer;
use std::env;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // let mut parser = parser::Parser::new("examples/StackTest.vm").unwrap();
    // let mut parser = parser::Parser::new("examples/SimpleSub.vm").unwrap();
    let mut parser = parser::Parser::new(args[1].as_str()).unwrap();
    let mut code_writer = code_writer::CodeWriter::new("tmp.txt").unwrap();
    while parser.has_more_lines().unwrap() {
        // parser.advance();
        // なんかやる
        let line = parser.next_line();
        eprintln!("{}", line);
        if parser.command_type() == parser::CommandType::C_INIT {
            break;
        }
        // code_writer.write_arithmetic(line.as_str());
        code_writer.write_comment(parser.command_type(), parser.arg1(), parser.arg2());
        code_writer.write_code(parser.command_type(), parser.arg1(), parser.arg2());
    }
}

