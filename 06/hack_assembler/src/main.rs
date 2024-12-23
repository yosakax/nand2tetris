use std::env;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

pub mod code;
pub mod parser;
pub mod symbol_table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let src = args[1].to_owned();
    let target_name = Path::new(&args[1])
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if !src.ends_with(".asm") {
        return Err(format!("This file is not hack assembly: {}", src).into());
    }
    let codegen = code::CodeGen::new();

    let mut target = BufWriter::new(fs::File::create(format!("./results/{}.hack", target_name))?);

    let mut parser = parser::Parser::new(&src);

    while parser.has_more_lines() {
        parser.advance();
        // eprintln!("i = {:06b}", parser.lineno);
        // eprintln!(
        //     "dest = {}, comp = {}, jump = {}, symbol = {}",
        //     parser.dest, parser.comp, parser.jump, parser.symbol
        // );

        // eprintln!("{}", parser.codes[parser.lineno]);
        let code = match parser.instruction_type {
            parser::InstructionType::A_INSTRUCTION => {
                let symbol: usize = parser.symbol.parse().unwrap();
                let code = format!("0{:015b}", symbol);
                // println!("{}", code);
                code
            }
            parser::InstructionType::C_INSTRUCTION => {
                let dest_code = codegen.gen_dest(&parser.dest);
                let comp_code = codegen.gen_comp(&parser.comp);
                let jump_code = codegen.gen_jump(&parser.jump);
                let abit = codegen.gen_abit(&parser.comp);
                let code = format!("111{}{}{}{}", abit, comp_code, dest_code, jump_code);
                // println!("{}", code);
                code
            }
            parser::InstructionType::L_INSTRUCTION => {
                unimplemented!();
            }
        };
        println!("{}", code);
        target.write(format!("{}\n", code).as_bytes()).unwrap();
    }

    Ok(())
}

