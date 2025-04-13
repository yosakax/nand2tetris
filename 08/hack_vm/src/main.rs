mod code_writer;
use std::{env, fs};
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_path = args[1].as_str();
    // ディレクトリならファイルの一覧、ファイルならそのファイルの配列を作る
    let mut files: Vec<String> = vec![];
    if fs::metadata(arg_path).unwrap().is_dir() {
        let dir = fs::read_dir(arg_path).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().unwrap() == "vm" {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    } else {
        files.push(arg_path.to_string());
    }

    println!("{:?}", files);

    // 最終的にはすべてのファイルをtmp.asmに書き出す
    let mut code_writer = code_writer::CodeWriter::new("tmp.asm").unwrap();
    for file_name in files.iter() {
        // let mut parser = parser::Parser::new(args[1].as_str()).unwrap();
        code_writer.set_base_name(file_name);
        let mut parser = parser::Parser::new(file_name).unwrap();
        while parser.has_more_lines().unwrap() {
            let line = parser.next_line();
            eprintln!("{}", line);
            if parser.command_type() == parser::CommandType::C_INIT {
                break;
            }
            code_writer.write_comment(parser.command_type(), parser.arg1(), parser.arg2());
            code_writer.write_code(parser.command_type(), parser.arg1(), parser.arg2());
        }
    }
}

