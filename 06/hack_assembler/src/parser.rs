use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum InstructionType {
    A_INSTRUCTION,
    C_INSTRUCTION,
    L_INSTRUCTION,
}

/// 入力されたアゼンブリコードへの便利なアクセスを提供する
#[derive(Debug, Clone)]
pub struct Parser {
    pub codes: Vec<String>,
    pub lineno: usize,
    pub dest: String,
    pub comp: String,
    pub jump: String,
    pub symbol: String,
    pub instruction_type: InstructionType,
}

impl Parser {
    pub fn new(src_filepath: &String) -> Self {
        let mut f = File::open(src_filepath).expect("cannot open file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        let lineno = 0usize;
        let codes: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
        let dest = String::new();
        let comp = String::new();
        let jump = String::new();
        let symbol = String::new();

        Parser {
            codes,
            lineno,
            dest,
            comp,
            jump,
            symbol,
            instruction_type: InstructionType::A_INSTRUCTION,
        }
    }

    pub fn has_more_lines(&self) -> bool {
        self.lineno + 1 <= self.codes.len()
    }

    fn is_valid(&self) -> bool {
        !self.codes[self.lineno].is_empty() && !self.codes[self.lineno].starts_with("//")
    }

    pub fn advance(&mut self) {
        // while self.has_more_lines() {
        //     self.lineno += 1;
        // }
        while !self.is_valid() {
            self.lineno += 1;
        }
        // let line = self.codes[self.lineno].to_owned();
        let lines: Vec<&str> = self.codes[self.lineno].split("//").collect();
        let line = lines[0].to_string();

        if line.starts_with("(") {
            // Symbolのときのパーサを書く
            self.instruction_type = InstructionType::L_INSTRUCTION;
            let re = Regex::new(r"(?<=\()<target>[A-Z,0-9]*(?=\))").unwrap();
            let caps = re.captures(line.as_str()).unwrap();
            self.symbol = caps.name("target").map_or("", |m| m.as_str()).to_string();
        } else if line.starts_with("@") {
            // @のときのパーサを書く
            self.instruction_type = InstructionType::A_INSTRUCTION;
            self.symbol = line[1..].to_string();
        } else {
            // 式
            self.instruction_type = InstructionType::C_INSTRUCTION;
            if line.contains("=") {
                let lines: Vec<&str> = line.split("=").collect();
                self.dest = lines[0].trim().to_string();
                self.comp = lines[1].trim().to_string();
                self.jump = "null".to_string();
            } else if line.contains(";") {
                let lines: Vec<&str> = line.split(";").collect();
                self.dest = "null".to_string();
                self.comp = lines[0].trim().to_string();
                self.jump = lines[1].trim().to_string();
            }
        }
        self.lineno += 1;
        // println!("code:\n{}", line);
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.to_owned()
    }
    pub fn get_dest(&self) -> String {
        assert_eq!(self.instruction_type, InstructionType::C_INSTRUCTION);
        self.dest.to_owned()
    }
    pub fn get_comp(&self) -> String {
        assert_eq!(self.instruction_type, InstructionType::C_INSTRUCTION);
        self.comp.to_owned()
    }
    pub fn get_jump(&self) -> String {
        assert_eq!(self.instruction_type, InstructionType::C_INSTRUCTION);
        self.jump.to_owned()
    }
    pub fn get_instruction_type(&self) -> InstructionType {
        self.instruction_type.to_owned()
    }
}

