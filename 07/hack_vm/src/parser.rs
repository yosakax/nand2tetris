use std::fmt;
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
            "add" | "sub" | "neg" | "and" | "or" | "not" | "eq" | "gt" | "lt" => {
                self.command_type = CommandType::C_ARITHETIC;
                self.arg1 = split_command[0].to_string();
                self.arg2 = None;
            }
            "" => {
                self.command_type = CommandType::C_INIT;
                self.arg1 = String::new();
                self.arg2 = None;
            }
            _ => {
                eprintln!("{:?}", split_command);
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
    pub fn command_type(&self) -> CommandType {
        self.command_type.clone()
    }
    pub fn arg1(&self) -> String {
        self.arg1.clone()
    }
    pub fn arg2(&self) -> Option<usize> {
        self.arg2.clone()
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CommandType {
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

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::C_INIT => write!(f, "C_INIT"),
            CommandType::C_ARITHETIC => write!(f, "C_ARITHETIC"),
            CommandType::C_PUSH => write!(f, "C_PUSH"),
            CommandType::C_POP => write!(f, "C_POP"),
            CommandType::C_LABEL => write!(f, "C_LABEL"),
            CommandType::C_GOTO => write!(f, "C_GOTO"),
            CommandType::C_IF => write!(f, "C_IF"),
            CommandType::C_FUNCTION => write!(f, "C_FUNCTION"),
            CommandType::C_RETURN => write!(f, "C_RETURN"),
            CommandType::C_CALL => write!(f, "C_CALL"),
        }
    }
}

