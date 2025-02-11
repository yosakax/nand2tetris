use crate::parser::CommandType;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    pub stream: BufWriter<File>,
    label_number: usize,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let file = File::create(file_path)?;
        let stream = BufWriter::new(file);
        return Ok(CodeWriter {
            stream,
            label_number: 0,
        });
    }

    pub fn write_code(&mut self, command: CommandType, arg1: String, arg2: Option<usize>) {
        match command {
            CommandType::C_POP => {
                self.write_pop(arg1, arg2);
            }
            CommandType::C_PUSH => {
                self.write_push(arg1, arg2);
            }
            CommandType::C_ARITHETIC => {
                self.write_arithmetic(command, arg1);
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub fn write_arithmetic(&mut self, command: CommandType, arg1: String) {
        match &*arg1 {
            "add" => self.write_add(),
            "sub" => self.write_sub(),
            "neg" => self.write_neg(),
            "and" => self.write_and(),
            "or" => self.write_or(),
            "not" => self.write_not(),
            "eq" => self.write_eq(),
            "lt" => self.write_lt(),
            "gt" => self.write_gt(),
            _ => {
                unimplemented!();
            }
        }
    }

    /// 作れって指示あったけど必要か？
    pub fn write_push_pop(&mut self, command: CommandType, segment: &String, index: usize) {}

    pub fn write_comment(&mut self, command: CommandType, arg1: String, arg2: Option<usize>) {
        let comment = format!(
            "// {}, {}, {}",
            command.to_string(),
            arg1,
            arg2.unwrap_or(0)
        );
        self.stream.write_all(comment.as_bytes()).unwrap();
        self.write_line_break();
    }

    pub fn write_simple_comment(&mut self, comment: &str) {
        let comment = format!("//{}", comment);
        self.stream.write_all(comment.as_bytes()).unwrap();
        self.write_line_break();
    }

    fn get_label_name(&mut self) -> String {
        let label_name = format!("L{}", self.label_number);
        self.label_number += 1;
        label_name
    }

    /// 改行を追加するだけ
    fn write_line_break(&mut self) {
        self.stream.write_all(b"\n").unwrap();
        self.stream.flush().unwrap();
    }

    /// スタックの先頭の値をpopして、segment[index](arg1[arg2])に格納する
    fn write_pop(&mut self, arg1: String, arg2: Option<usize>) {
        let address = self.get_index(arg1.clone(), arg2.clone());
        eprintln!("arg1 = {}, index = {}", arg1, address);
        let address = format!("@{}", address);
        // データを取ってくる
        self.write_simple_comment("start pop");
        self.write(vec!["@SP", "M=M-1", "A=M"]);
        self.write(vec!["D=M", address.as_str(), "M=D"]);
        self.write_simple_comment("end pop");
    }

    /// segment[index](arg1[arg2])の値をスタックにpushする
    fn write_push(&mut self, arg1: String, arg2: Option<usize>) {
        let address = self.get_index(arg1.clone(), arg2.clone());
        eprintln!("arg1 = {}, index = {}", arg1, address);
        let address = format!("@{}", address);
        // データ取ってくる
        if arg1.as_str() == "constant" {
            self.write(vec![address.as_str(), "D=A"]);
        } else {
            self.write(vec![address.as_str(), "D=M"]);
        }
        self.write(vec!["@SP", "A=M", "M=D", "@SP", "M=M+1"]);
    }

    // stackから2つ持ってきて足す
    fn write_add(&mut self) {
        self.write_simple_comment("start add");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "M=D+M"]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end add");
    }

    fn write_sub(&mut self) {
        self.write_simple_comment("start sub");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "M=D-M"]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end sub");
    }
    fn write_neg(&mut self) {
        self.write_simple_comment("start neg");
        self.write_pop("this".to_string(), None);
        let index = self.get_index("pointer".to_string(), Some(0));
        let address = format!("@{}", index);
        // self.write(vec!["@0", "D=A", address.as_str(), "M=D-M"]);
        self.write(vec![address.as_str(), "M=-M"]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end neg");
    }

    fn write_and(&mut self) {
        self.write_simple_comment("start and");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "D=D&M", "M=D"]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end and");
    }

    fn write_or(&mut self) {
        self.write_simple_comment("start or");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "D=D|M", "M=D"]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end or");
    }

    fn write_not(&mut self) {
        self.write_simple_comment("start not");
        self.write_pop("this".to_string(), None);
        let index = self.get_index("pointer".to_string(), Some(0));
        let address = format!("@{}", index);
        // self.write(vec!["@0", "D=A", address.as_str(), "M=D-M"]);
        self.write(vec![address.as_str(), "M=!M"]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end not");
    }

    fn write_eq(&mut self) {
        self.write_simple_comment("start eq");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        let label_true = self.get_label_name();
        let label_false = self.get_label_name();
        let next_label = self.get_label_name();
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "D=D-M"]);
        self.write(vec![
            format!("@{}", label_true).as_str(),
            "D;JEQ",
            format!("@{}", label_false).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", label_true).as_str(),
            "D=-1",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", label_false).as_str(),
            "D=0",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", next_label).as_str(),
            arg1_address.as_str(),
            "M=D",
        ]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end eq")
    }

    fn write_lt(&mut self) {
        self.write_simple_comment("start lt");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        let label_true = self.get_label_name();
        let label_false = self.get_label_name();
        let next_label = self.get_label_name();
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "D=D-M"]);
        self.write(vec![
            format!("@{}", label_true).as_str(),
            "D;JLT",
            format!("@{}", label_false).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", label_true).as_str(),
            "D=-1",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", label_false).as_str(),
            "D=0",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", next_label).as_str(),
            arg1_address.as_str(),
            "M=D",
        ]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end lt")
    }

    fn write_gt(&mut self) {
        self.write_simple_comment("start gt");
        self.write_pop("this".to_string(), None);
        self.write_pop("that".to_string(), None);
        let arg1_index = self.get_index("pointer".to_string(), Some(0));
        let arg2_index = self.get_index("pointer".to_string(), Some(1));
        let arg1_address = format!("@{}", arg1_index);
        let arg2_address = format!("@{}", arg2_index);
        let label_true = self.get_label_name();
        let label_false = self.get_label_name();
        let next_label = self.get_label_name();
        self.write(vec![arg2_address.as_str(), "D=M"]);
        self.write(vec![arg1_address.as_str(), "D=D-M"]);
        self.write(vec![
            format!("@{}", label_true).as_str(),
            "D;JGT",
            format!("@{}", label_false).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", label_true).as_str(),
            "D=-1",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", label_false).as_str(),
            "D=0",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(vec![
            format!("({})", next_label).as_str(),
            arg1_address.as_str(),
            "M=D",
        ]);
        self.write_push("this".to_string(), None);
        self.write_simple_comment("end gt")
    }

    fn write(&mut self, commands: Vec<&str>) {
        let command = commands.join("\n");
        self.stream.write_all(command.as_bytes()).unwrap();
        self.write_line_break();
    }

    fn get_index(&self, arg1: String, arg2: Option<usize>) -> usize {
        let address = match &*arg1 {
            "local" | "argument" | "this" | "that" => match &*arg1 {
                "argument" => VmAddress::ARG.as_usize() + arg2.unwrap_or(0),
                "local" => VmAddress::LCL.as_usize() + arg2.unwrap_or(0),
                "this" => VmAddress::THIS.as_usize() + arg2.unwrap_or(0),
                "that" => VmAddress::THAT.as_usize() + arg2.unwrap_or(0),
                _ => {
                    unreachable!();
                }
            },

            "static" => VmAddress::Static.as_usize() + arg2.unwrap_or(0),
            "constant" => arg2.unwrap_or(0),
            "pointer" => match arg2.unwrap_or(0) {
                0 => VmAddress::THIS.as_usize(),
                1 => VmAddress::THAT.as_usize(),
                _ => {
                    unreachable!();
                }
            },
            "temp" => VmAddress::TEMP.as_usize() + arg2.unwrap_or(0),
            _ => {
                unreachable!();
            }
        };
        address
    }

    fn get_dynamic_index(&mut self, arg1: String, arg2: Option<usize>) {}
}

#[derive(Copy, Clone)]
enum VmAddress {
    SP = 0,
    LCL = 1,
    ARG = 2,
    THIS = 3,
    THAT = 4,
    TEMP = 5,
    // R0 = 0,
    // R1 = 1,
    // R2 = 2,
    // R3 = 3,
    // R4 = 4,
    // R5 = 5,
    // R6 = 6,
    // R7 = 7,
    // R8 = 8,
    // R9 = 9,
    // R10 = 10,
    // R11 = 11,
    // R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    Static = 16,
}

impl VmAddress {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone)]
enum MemorySegment {
    Argument = 400,
    Local = 300,
    Pointer = 3,
    Temp = 5,
    Constant = 0,
    Static = 16,
}

impl MemorySegment {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

// impl fmt::Display for MemorySegment {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MemorySegment::Local => write!(f, "local"),
//             MemorySegment::Temp => write!(f, "temp"),
//             MemorySegment::Pointer => write!(f, "pointer"),
//             MemorySegment::Constant => write!(f, "constant"),
//             MemorySegment::Static => write!(f, "static"),
//         }
//     }
// }

