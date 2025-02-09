use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

pub struct Parser {
    pub stream: BufReader<File>,
    command_type: CommandType,
    arg1: String,
    arg2: Option<usize>,
}

impl Parser {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        Ok(Parser {
            stream: reader,
            command_type: CommandType::C_INIT,
            arg1: String::new(),
            arg2: None,
        })
    }

    pub fn next_line(&mut self) -> String {
        let mut line = String::new();
        self.stream.read_line(&mut line).unwrap();
        line = line.trim().to_string();
        while line.is_empty() || line.starts_with("//") {
            line.clear();
            let res = self.stream.read_line(&mut line).unwrap();
            if res == 0 {
                break;
            }
            line = line.trim().to_string();
        }
        self.parse_code(line.as_str());

        return line.clone();
    }

    fn parse_code(&mut self, command: &str) {
        let split_command: Vec<&str> = command.split(" ").collect();
        match split_command[0] {
            "push" => {
                self.command_type = CommandType::C_PUSH;
                self.arg1 = split_command[1].to_string();
                self.arg2 = Some(split_command[2].parse::<usize>().unwrap());
            }
            "pop" => {
                self.command_type = CommandType::C_POP;
                self.arg1 = split_command[1].to_string();
                self.arg2 = Some(split_command[2].parse::<usize>().unwrap());
            }
            "add" | "sub" | "neg" | "and" | "or" | "not" => {
                self.command_type = CommandType::C_ARITHETIC;
            }
            _ => {
                unreachable!()
            }
        }
        // if split_command[0] == "push" {
        //     self.command_type = CommandType::C_PUSH;
        //     self.arg1 = split_command[1].to_string();
        //     self.arg2 = Some(split_command[2].parse::<usize>().unwrap());
        // }
        // else if split_command[0] == "pop" {
        //     self.command_type = CommandType::C_POP;
        //     self.arg1 = split_command[1].to_string();
        //     self.arg2 = Some(split_command[2].parse::<usize>().unwrap());
        // }

        // if split_command[0] == "add"{

        // }else if split_command[0] == "sub"{

        // }else if split_command[0] == "neg"{

        // }

        // if split_command[0] == ""
    }

    pub fn has_more_lines(&mut self) -> std::io::Result<bool> {
        let current_pos = self.stream.stream_position().unwrap();
        let mut tmp = String::new();
        match self.stream.read_line(&mut tmp) {
            Ok(0) => {
                return Ok(false);
            }
            Ok(_) => {
                self.stream
                    .seek(std::io::SeekFrom::Start(current_pos))
                    .unwrap();
                return Ok(true);
            }
            Err(e) => Err(e),
        }
    }

    /// 次の入力コマンドを読み込んで、現在のコマンドとする
    pub fn advance(&mut self) {
        // self.current_position.unwrap() += 1;
    }
}

enum CommandType {
    C_INIT, // 初期値
    C_ARITHETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}

#[derive(Copy, Clone)]
enum VmAddress {
    SP = 0,
    LCL = 1,
    ARG = 2,
    THIS = 3,
    THAT = 4,
    TEMP = 5,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl VmAddress {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

