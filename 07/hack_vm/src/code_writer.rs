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
            // "lt" => self.write_lt(),
            // "gt" => self.write_gt(),
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
        self.write_simple_comment("start pop");
        self.load_symbol("@SP".to_string());
        self.load_m("M-1");
        self.load_a("M");
        self.load_d("M");
        self.load_m_from_args(arg1.clone(), arg2.clone());
        self.load_m("D");
        self.write_simple_comment("end pop");
    }

    /// segment[index](arg1[arg2])の値をスタックにpushする
    fn write_push(&mut self, arg1: String, arg2: Option<usize>) {
        self.load_d_from_args(arg1.clone(), arg2.clone());
        self.load_symbol("@SP".to_string());
        self.load_a("M");
        self.load_m("D");
        self.load_symbol("@SP".to_string());
        self.load_m("M+1");
        // self.write(vec!["@SP", "A=M", "M=D", "@SP", "M=M+1"]);
    }

    // stackから2つ持ってきて足す
    fn write_add(&mut self) {
        self.write_simple_comment("start add");
        self.write_pop("temp".to_string(), Some(0));
        self.write_pop("temp".to_string(), Some(1));
        self.load_d_from_args("temp".to_string(), Some(0));
        self.load_m_from_args("temp".to_string(), Some(1));
        self.load_m("D+M");
        self.write_push("temp".to_string(), Some(1));
        self.write_simple_comment("end add");
    }

    fn write_sub(&mut self) {
        self.write_simple_comment("start sub");
        self.write_pop("temp".to_string(), Some(0));
        self.write_pop("temp".to_string(), Some(1));
        self.load_d_from_args("temp".to_string(), Some(1));
        self.load_m_from_args("temp".to_string(), Some(0));
        self.load_m("D-M");
        self.write_push("temp".to_string(), Some(0));
        self.write_simple_comment("end sub");
    }

    fn write_neg(&mut self) {
        self.write_simple_comment("start neg");
        self.write_pop("temp".to_string(), None);
        self.load_m_from_args("temp".to_string(), None);
        self.load_m("-M");
        self.write_push("temp".to_string(), None);
        self.write_simple_comment("end neg");
    }

    fn write_and(&mut self) {
        self.write_simple_comment("start and");
        self.write_pop("temp".to_string(), Some(0));
        self.write_pop("temp".to_string(), Some(1));
        self.load_d_from_args("temp".to_string(), Some(1));
        self.load_m_from_args("temp".to_string(), Some(0));
        self.load_d("D&M");
        self.load_m("D");
        self.write_push("temp".to_string(), Some(0));
        self.write_simple_comment("end and");
    }

    fn write_or(&mut self) {
        self.write_simple_comment("start or");
        self.write_pop("temp".to_string(), Some(0));
        self.write_pop("temp".to_string(), Some(1));
        self.load_d_from_args("temp".to_string(), Some(1));
        self.load_m_from_args("temp".to_string(), Some(0));
        self.load_d("D|M");
        self.load_m("D");
        self.write_push("temp".to_string(), Some(0));
        self.write_simple_comment("end or");
    }

    fn write_not(&mut self) {
        self.write_simple_comment("start not");
        self.write_pop("temp".to_string(), Some(0));
        self.load_m_from_args("temp".to_string(), Some(0));
        self.load_m("!M");
        self.write_push("temp".to_string(), Some(0));
        self.write_simple_comment("end not");
    }

    fn write_eq(&mut self) {
        unimplemented!();
        // self.write_simple_comment("start eq");
        // self.write_pop("temp".to_string(), Some(0));
        // self.write_pop("temp".to_string(), Some(1));
        // let arg1_index = self.get_index("temp".to_string(), Some(0));
        // let arg2_index = self.get_index("temp".to_string(), Some(1));
        // let arg1_address = format!("@{}", arg1_index);
        // let arg2_address = format!("@{}", arg2_index);
        // let label_true = self.get_label_name();
        // let label_false = self.get_label_name();
        // let next_label = self.get_label_name();
        // self.write(vec![arg2_address.as_str(), "D=M"]);
        // self.write(vec![arg1_address.as_str(), "D=D-M"]);
        // self.write(vec![
        //     format!("@{}", label_true).as_str(),
        //     "D;JEQ",
        //     format!("@{}", label_false).as_str(),
        //     "0;JMP",
        // ]);
        // self.write(vec![
        //     format!("({})", label_true).as_str(),
        //     "D=-1",
        //     format!("@{}", next_label).as_str(),
        //     "0;JMP",
        // ]);
        // self.write(vec![
        //     format!("({})", label_false).as_str(),
        //     "D=0",
        //     format!("@{}", next_label).as_str(),
        //     "0;JMP",
        // ]);
        // self.write(vec![
        //     format!("({})", next_label).as_str(),
        //     arg1_address.as_str(),
        //     "M=D",
        // ]);
        // self.write_push("temp".to_string(), Some(0));
        // self.write_simple_comment("end eq")
    }

    // fn write_lt(&mut self) {
    //     self.write_simple_comment("start lt");
    //     self.write_pop("temp".to_string(), Some(0));
    //     self.write_pop("temp".to_string(), Some(1));
    //     let arg1_index = self.get_index("temp".to_string(), Some(0));
    //     let arg2_index = self.get_index("temp".to_string(), Some(1));
    //     let arg1_address = format!("@{}", arg1_index);
    //     let arg2_address = format!("@{}", arg2_index);
    //     let label_true = self.get_label_name();
    //     let label_false = self.get_label_name();
    //     let next_label = self.get_label_name();
    //     self.write(vec![arg2_address.as_str(), "D=M"]);
    //     self.write(vec![arg1_address.as_str(), "D=D-M"]);
    //     self.write(vec![
    //         format!("@{}", label_true).as_str(),
    //         "D;JLT",
    //         format!("@{}", label_false).as_str(),
    //         "0;JMP",
    //     ]);
    //     self.write(vec![
    //         format!("({})", label_true).as_str(),
    //         "D=-1",
    //         format!("@{}", next_label).as_str(),
    //         "0;JMP",
    //     ]);
    //     self.write(vec![
    //         format!("({})", label_false).as_str(),
    //         "D=0",
    //         format!("@{}", next_label).as_str(),
    //         "0;JMP",
    //     ]);
    //     self.write(vec![
    //         format!("({})", next_label).as_str(),
    //         arg1_address.as_str(),
    //         "M=D",
    //     ]);
    //     self.write_push("temp".to_string(), Some(0));
    //     self.write_simple_comment("end lt")
    // }

    // fn write_gt(&mut self) {
    //     self.write_simple_comment("start gt");
    //     self.write_pop("this".to_string(), None);
    //     self.write_pop("that".to_string(), None);
    //     let arg1_index = self.get_index("pointer".to_string(), Some(0));
    //     let arg2_index = self.get_index("pointer".to_string(), Some(1));
    //     let arg1_address = format!("@{}", arg1_index);
    //     let arg2_address = format!("@{}", arg2_index);
    //     let label_true = self.get_label_name();
    //     let label_false = self.get_label_name();
    //     let next_label = self.get_label_name();
    //     self.write(vec![arg2_address.as_str(), "D=M"]);
    //     self.write(vec![arg1_address.as_str(), "D=D-M"]);
    //     self.write(vec![
    //         format!("@{}", label_true).as_str(),
    //         "D;JGT",
    //         format!("@{}", label_false).as_str(),
    //         "0;JMP",
    //     ]);
    //     self.write(vec![
    //         format!("({})", label_true).as_str(),
    //         "D=-1",
    //         format!("@{}", next_label).as_str(),
    //         "0;JMP",
    //     ]);
    //     self.write(vec![
    //         format!("({})", label_false).as_str(),
    //         "D=0",
    //         format!("@{}", next_label).as_str(),
    //         "0;JMP",
    //     ]);
    //     self.write(vec![
    //         format!("({})", next_label).as_str(),
    //         arg1_address.as_str(),
    //         "M=D",
    //     ]);
    //     self.write_push("temp".to_string(), Some(0));
    //     self.write_simple_comment("end gt")
    // }

    fn write(&mut self, command: &str) {
        self.stream.write_all(command.as_bytes()).unwrap();
        self.write_line_break();
    }

    fn write_multiple(&mut self, commands: Vec<&str>) {
        let command = commands.join("\n");
        self.stream.write_all(command.as_bytes()).unwrap();
        self.write_line_break();
    }

    fn load_a(&mut self, arg: &str) {
        self.write(format!("A={}", arg).as_str());
    }
    fn load_m(&mut self, arg: &str) {
        self.write(format!("M={}", arg).as_str());
    }
    fn load_d(&mut self, arg: &str) {
        self.write(format!("D={}", arg).as_str());
    }
    fn load_symbol(&mut self, arg: String) {
        self.write(arg.as_str());
    }

    fn load_d_from_args(&mut self, arg1: String, arg2: Option<usize>) {
        match &*arg1 {
            "local" | "argument" | "this" | "that" => match &*arg1 {
                /* 動的にメモリの位置が変更される */
                "argument" => self
                    .load_d_from_dynamic_index_value(VmAddress::ARG.as_usize(), arg2.unwrap_or(0)),
                "local" => self
                    .load_d_from_dynamic_index_value(VmAddress::LCL.as_usize(), arg2.unwrap_or(0)),
                "this" => self
                    .load_d_from_dynamic_index_value(VmAddress::THIS.as_usize(), arg2.unwrap_or(0)),
                "that" => self
                    .load_d_from_dynamic_index_value(VmAddress::THAT.as_usize(), arg2.unwrap_or(0)),
                _ => {
                    unreachable!();
                }
            },
            "static" => {
                self.load_d_from_static_index_value(VmAddress::STATIC.as_usize(), arg2.unwrap_or(0))
            }
            "constant" => {
                self.write_multiple(vec![format!("@{}", arg2.unwrap_or(0)).as_str(), "D=A"]);
            }
            "pointer" => match arg2.unwrap_or(0) {
                0 => self.load_d_from_dynamic_index_value(VmAddress::THIS.as_usize(), 0),
                1 => self.load_d_from_dynamic_index_value(VmAddress::THAT.as_usize(), 0),
                _ => unreachable!(),
            },
            "temp" => {
                self.load_d_from_static_index_value(VmAddress::TEMP.as_usize(), arg2.unwrap_or(0))
            }
            _ => {
                unreachable!();
            }
        };
    }

    fn load_m_from_args(&mut self, arg1: String, arg2: Option<usize>) {
        match &*arg1 {
            "local" | "argument" | "this" | "that" => match &*arg1 {
                /* 動的にメモリの位置が変更される */
                "argument" => self
                    .load_m_from_dynamic_index_value(VmAddress::ARG.as_usize(), arg2.unwrap_or(0)),
                "local" => self
                    .load_m_from_dynamic_index_value(VmAddress::LCL.as_usize(), arg2.unwrap_or(0)),
                "this" => self.load_m_from_dynamic_index_value(VmAddress::THIS.as_usize(), 0),
                "that" => self.load_m_from_dynamic_index_value(VmAddress::THAT.as_usize(), 0),
                _ => {
                    unreachable!();
                }
            },
            "static" => {
                self.load_m_from_static_index_value(VmAddress::STATIC.as_usize(), arg2.unwrap_or(0))
            }
            "pointer" => match arg2.unwrap_or(0) {
                0 => self.load_m_from_dynamic_index_value(VmAddress::THIS.as_usize(), 0),
                1 => self.load_m_from_dynamic_index_value(VmAddress::THAT.as_usize(), 0),
                _ => unreachable!(),
            },
            "temp" => {
                self.load_m_from_static_index_value(VmAddress::TEMP.as_usize(), arg2.unwrap_or(0))
            }
            _ => {
                unreachable!();
            }
        };
    }

    /// Dレジスタに指定したアドレスのデータをロードする
    fn load_d_from_dynamic_index_value(&mut self, base_address: usize, offset: usize) {
        self.load_m_from_dynamic_index_value(base_address, offset);
        self.load_d("M");
    }

    fn load_m_from_dynamic_index_value(&mut self, base_address: usize, offset: usize) {
        let base_symbol = format!("@{}", base_address);
        let offset_symbol = format!("@{}", offset);
        self.load_symbol(offset_symbol);
        self.load_d("A");
        self.load_symbol(base_symbol);
        self.load_a("M+D");
    }

    /// Dレジスタに指定したアドレスのデータをロードする
    fn load_d_from_static_index_value(&mut self, base_address: usize, offset: usize) {
        self.load_m_from_static_index_value(base_address, offset);
        self.load_d("M");
    }

    fn load_m_from_static_index_value(&mut self, base_address: usize, offset: usize) {
        let index_symbol = format!("@{}", base_address + offset);
        self.load_symbol(index_symbol);
    }
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
    STATIC = 16,
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

