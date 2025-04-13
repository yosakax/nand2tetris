use crate::parser::CommandType;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    pub stream: BufWriter<File>,
    pub base_name: String,
    label_number: usize,
    function_name: String,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let file = File::create(file_path)?;
        let stream = BufWriter::new(file);
        let base_name = String::new();
        return Ok(CodeWriter {
            stream,
            base_name,
            label_number: 0,
            function_name: String::new(),
        });
    }

    pub fn set_base_name(&mut self, file_path: &str) {
        let base_name = file_path.split('/').next().unwrap().to_string();
        self.base_name = base_name;
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
            CommandType::C_LABEL => {
                self.write_label(arg1);
            }
            CommandType::C_IF => {
                self.write_if_goto(arg1);
            }
            CommandType::C_GOTO => {
                self.write_goto(arg1);
            }
            CommandType::C_FUNCTION => {
                self.write_function(arg1, arg2);
            }
            CommandType::C_RETURN => {
                self.write_return();
            }
            CommandType::C_CALL => {
                self.write_call(arg1, arg2);
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub fn write_arithmetic(&mut self, _command: CommandType, arg1: String) {
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
            "label" => self.write_label(arg1),
            "if-goto" => self.write_if_goto(arg1),
            "goto" => self.write_goto(arg1),
            _ => {
                unimplemented!();
            }
        }
    }

    /// 関数宣言
    /// n_varsのローカル変数を0で初期化する
    ///
    /// * `function_name`: - 関数の名前
    /// * `n_vars`: - ローカル変数の数
    fn write_function(&mut self, function_name: String, n_vars: Option<usize>) {
        self.write_simple_comment("start function");
        self.write(format!("({}.{})", self.base_name, function_name).as_str());
        for i in 0..n_vars.unwrap_or(0) {
            self.write_push("constant".to_string(), Some(0));
            self.write_pop("local".to_string(), Some(i));
        }
        self.write_simple_comment("end function");
    }

    fn write_return(&mut self) {
        self.write_simple_comment("start return");
        // ARG, SP, THAT, THIS, ARG, LCLを復元

        // ARG
        self.write_pop("argument".to_string(), None);

        // return アドレスに移動

        self.write_simple_comment("end return");
    }

    ///  関数呼び出し
    ///
    /// * `function_name`: - 関数の名前
    /// * `n_args`: - stackに積まれている引数の数
    fn write_call(&mut self, function_name: String, n_args: Option<usize>) {
        self.write_simple_comment("start call function");
        /* こんな感じでスタックに積む
         * push return return address
         * push LCL
         * push arg
         * push this
         * push that
         * arg = sp - 5 - nargs
         * lcl = sp
         * goto f
         * (return address)*/

        // リターンアドレスのpush
        let return_label = self.get_new_return_label();
        self.write_multiple(vec![format!("@{}", return_label).as_str(), "D=A"]);
        self.write_push_from_d();

        // localのpush
        self.load_d_from_args("LCL".to_string(), None);
        self.write_push_from_d();

        // argumentのpush
        self.load_d_from_args("ARG".to_string(), None);
        self.write_push_from_d();

        // THISのpush
        self.load_d_from_args("THIS".to_string(), None);
        self.write_push_from_d();

        // THATのpush
        self.load_d_from_args("THAT".to_string(), None);
        self.write_push_from_d();

        // ARGの設定
        self.write_multiple(vec![
            "@SP",
            "D=A",
            "@5",
            "D=D-A",
            format!("@{}", n_args.unwrap()).as_str(),
            "D=D-A",
            "@ARG",
            "M=D",
        ]);

        // LCLの設定
        self.write_multiple(vec!["@SP", "D=A", "@LCL", "M=D"]);

        // goto function
        let function_label = format!("{}.{}", self.base_name, function_name);
        self.write_goto(function_label);

        self.write_label(return_label);

        self.write_simple_comment("end call function");
    }

    fn write_label(&mut self, label: String) {
        self.write_simple_comment("start label");
        self.write(format!("({})", label).as_str());
        self.write_simple_comment("start label");
    }

    fn write_if_goto(&mut self, label: String) {
        self.write_simple_comment("start if-goto");
        self.load_symbol("@SP".to_string());
        // 事実上のpop
        self.load_m("M-1");
        self.load_a("M");
        self.load_d("M");
        self.write(format!("@{}", label).as_str());
        self.write("D;JGT");
        self.write_simple_comment("end if-goto");
    }

    fn write_goto(&mut self, label: String) {
        self.write_simple_comment("start goto");
        self.write(format!("@{}", label).as_str());
        self.write("0;JMP");
        self.write_simple_comment("end goto");
    }

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

    // write_callのときのreturn labelを取得する
    fn get_new_return_label(&mut self) -> String {
        let return_label = format!("__RETURN__{}", self.get_label_name());
        return return_label;
    }

    ///
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
    ///
    /// * `arg1`: segment
    /// * `arg2`: index
    fn write_pop(&mut self, arg1: String, arg2: Option<usize>) {
        self.write_simple_comment("start pop");
        // ここでMに格納先アドレスが入れる
        self.load_m_from_args(arg1.clone(), arg2.clone());
        self.load_d("A");
        self.load_symbol("@15".to_string());
        self.load_m("D");

        // スタックのデータ取り出し
        self.load_symbol("@SP".to_string());
        self.load_m("M-1"); // M = M - 1
        self.load_a("M"); // A = M
        self.load_d("M"); // D = *SP

        self.load_symbol("@15".to_string());
        self.load_a("M");
        self.load_m("D");

        self.write_simple_comment("end pop");
    }

    /// segment[index](arg1[arg2])の値をスタックにpushする
    ///
    /// * `arg1`: segment
    /// * `arg2`: index
    fn write_push(&mut self, arg1: String, arg2: Option<usize>) {
        self.write_simple_comment("start push");
        self.load_d_from_args(arg1.clone(), arg2.clone());
        // self.load_symbol("@15".to_string());
        // self.load_m("D");

        self.load_symbol("@SP".to_string());
        self.load_a("M");
        self.load_m("D");
        self.load_symbol("@SP".to_string());
        self.load_m("M+1");
        self.write_simple_comment("end push");
    }

    // stackから2つ持ってきて足す
    fn write_add(&mut self) {
        self.write_simple_comment("start add");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        self.load_d_from_args("R13".to_string(), None);
        self.load_m_from_args("R14".to_string(), None);
        self.load_m("D+M");
        self.write_push("R14".to_string(), None);
        self.write_simple_comment("end add");
    }

    /// stackから2つ持ってきて引く
    fn write_sub(&mut self) {
        self.write_simple_comment("start sub");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        self.load_d_from_args("R14".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_m("D-M");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end sub");
    }

    /// stackから1つ持ってきて-1倍する
    fn write_neg(&mut self) {
        self.write_simple_comment("start neg");
        self.write_pop("R13".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_m("-M");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end neg");
    }

    /// stackから2つ持ってきてandする
    fn write_and(&mut self) {
        self.write_simple_comment("start and");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        self.load_d_from_args("R14".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_d("D&M");
        self.load_m("D");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end and");
    }

    /// stackから2つ持ってきてorを取る
    fn write_or(&mut self) {
        self.write_simple_comment("start or");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        self.load_d_from_args("R14".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_d("D|M");
        self.load_m("D");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end or");
    }

    /// スタックの先頭の値を取り出して、その値を反転してpushする
    fn write_not(&mut self) {
        self.write_simple_comment("start not");
        self.write_pop("R13".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_m("!M");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end not");
    }

    /// stackから2つ持ってきて等しいかどうかを判定する
    /// 等しい場合は-1を、そうでない場合は0をpushする
    fn write_eq(&mut self) {
        self.write_simple_comment("start eq");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        let label_true = self.get_label_name();
        let label_false = self.get_label_name();
        let next_label = self.get_label_name();
        self.load_d_from_args("R13".to_string(), None);
        self.load_m_from_args("R14".to_string(), None);
        self.load_d("D-M");
        self.write_multiple(vec![
            format!("@{}", label_true).as_str(),
            "D;JEQ",
            format!("@{}", label_false).as_str(),
            "0;JMP",
        ]);
        self.write_multiple(vec![
            format!("({})", label_true).as_str(),
            "D=-1",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write_multiple(vec![
            format!("({})", label_false).as_str(),
            "D=0",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(format!("({})", next_label).as_str());
        self.write("@13");
        self.load_m("D");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end eq")
    }

    /// stackから2つ持ってきて、その大小を比較する (x < y)
    /// x < y の場合は-1を、そうでない場合は0をpushする
    fn write_lt(&mut self) {
        self.write_simple_comment("start lt");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        let label_true = self.get_label_name();
        let label_false = self.get_label_name();
        let next_label = self.get_label_name();
        self.load_d_from_args("R14".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_d("D-M");
        self.write_multiple(vec![
            format!("@{}", label_true).as_str(),
            "D;JLT",
            format!("@{}", label_false).as_str(),
            "0;JMP",
        ]);
        self.write_multiple(vec![
            format!("({})", label_true).as_str(),
            "D=-1",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write_multiple(vec![
            format!("({})", label_false).as_str(),
            "D=0",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(format!("({})", next_label).as_str());
        self.write("@13");
        self.load_m("D");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end lt")
    }

    /// stackから2つ持ってきて、その大小を比較する (x > y)
    /// x > y の場合は-1を、そうでない場合は0をpushする
    fn write_gt(&mut self) {
        self.write_simple_comment("start gt");
        self.write_pop("R13".to_string(), None);
        self.write_pop("R14".to_string(), None);
        let label_true = self.get_label_name();
        let label_false = self.get_label_name();
        let next_label = self.get_label_name();
        self.load_d_from_args("R14".to_string(), None);
        self.load_m_from_args("R13".to_string(), None);
        self.load_d("D-M");
        self.write_multiple(vec![
            format!("@{}", label_true).as_str(),
            "D;JGT",
            format!("@{}", label_false).as_str(),
            "0;JMP",
        ]);
        self.write_multiple(vec![
            format!("({})", label_true).as_str(),
            "D=-1",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write_multiple(vec![
            format!("({})", label_false).as_str(),
            "D=0",
            format!("@{}", next_label).as_str(),
            "0;JMP",
        ]);
        self.write(format!("({})", next_label).as_str());
        self.write("@13");
        self.load_m("D");
        self.write_push("R13".to_string(), None);
        self.write_simple_comment("end gt")
    }

    fn write(&mut self, command: &str) {
        self.stream.write_all(command.as_bytes()).unwrap();
        self.write_line_break();
    }

    fn write_multiple(&mut self, commands: Vec<&str>) {
        let command = commands.join("\n");
        self.stream.write_all(command.as_bytes()).unwrap();
        self.write_line_break();
    }

    fn write_push_from_d(&mut self) {
        self.write_simple_comment("start push_from_d");
        self.write("@SP");
        self.load_m("M+1");
        self.write("A=M-1");
        self.load_m("D");
        self.write_simple_comment("end push_from_d");
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

    /// Dレジスタに指定したアドレスのデータをロードする
    ///
    /// * `arg1`: segment
    /// * `arg2`: index
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
                0 => self.load_d_from_static_index_value(VmAddress::THIS.as_usize(), 0),
                1 => self.load_d_from_static_index_value(VmAddress::THAT.as_usize(), 0),
                _ => unreachable!(),
            },
            "temp" => {
                self.load_d_from_static_index_value(VmAddress::TEMP.as_usize(), arg2.unwrap_or(0))
            }
            "R13" => self.load_d_from_static_index_value(VmAddress::R13.as_usize(), 0),
            "R14" => self.load_d_from_static_index_value(VmAddress::R14.as_usize(), 0),
            "R15" => self.load_d_from_static_index_value(VmAddress::R15.as_usize(), 0),
            _ => {
                unreachable!();
            }
        };
    }

    /// Mレジスタに指定したアドレスのデータをロードする
    ///
    /// * `arg1`: segment
    /// * `arg2`: index
    fn load_m_from_args(&mut self, arg1: String, arg2: Option<usize>) {
        match &*arg1 {
            "local" | "argument" | "this" | "that" => match &*arg1 {
                /* 動的にメモリの位置が変更される */
                "argument" => self
                    .load_m_from_dynamic_index_value(VmAddress::ARG.as_usize(), arg2.unwrap_or(0)),
                "local" => self
                    .load_m_from_dynamic_index_value(VmAddress::LCL.as_usize(), arg2.unwrap_or(0)),
                "this" => self
                    .load_m_from_dynamic_index_value(VmAddress::THIS.as_usize(), arg2.unwrap_or(0)),
                "that" => self
                    .load_m_from_dynamic_index_value(VmAddress::THAT.as_usize(), arg2.unwrap_or(0)),
                _ => {
                    unreachable!();
                }
            },
            "static" => {
                self.load_m_from_static_index_value(VmAddress::STATIC.as_usize(), arg2.unwrap_or(0))
            }
            "pointer" => match arg2.unwrap_or(0) {
                0 => self.load_m_from_static_index_value(VmAddress::THIS.as_usize(), 0),
                1 => self.load_m_from_static_index_value(VmAddress::THAT.as_usize(), 0),
                _ => unreachable!(),
            },
            "temp" => {
                self.load_m_from_static_index_value(VmAddress::TEMP.as_usize(), arg2.unwrap_or(0))
            }
            "R13" => self.load_m_from_static_index_value(VmAddress::R13.as_usize(), 0),
            "R14" => self.load_m_from_static_index_value(VmAddress::R14.as_usize(), 0),
            "R15" => self.load_m_from_static_index_value(VmAddress::R15.as_usize(), 0),

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

    /// Mレジスタに指定したアドレスのデータをロードする
    fn load_m_from_dynamic_index_value(&mut self, base_address: usize, offset: usize) {
        self.write_simple_comment("start load_m_from_dynamic_index_value");
        let base_symbol = format!("@{}", base_address);
        let offset_symbol = format!("@{}", offset);
        self.load_symbol(base_symbol);
        self.load_d("M");
        self.load_symbol(offset_symbol);
        self.load_a("D+A");
        self.write_simple_comment("end load_m_from_dynamic_index_value");
    }

    /// Dレジスタに指定したアドレスのデータをロードする
    fn load_d_from_static_index_value(&mut self, base_address: usize, offset: usize) {
        self.load_m_from_static_index_value(base_address, offset);
        self.load_d("M");
    }

    /// Mレジスタに指定したアドレスのデータをロードする
    fn load_m_from_static_index_value(&mut self, base_address: usize, offset: usize) {
        let index_symbol = format!("@{}", base_address + offset);
        self.load_symbol(index_symbol);
    }
}

#[derive(Copy, Clone)]
enum VmAddress {
    LCL = 1,
    ARG = 2,
    THIS = 3,
    THAT = 4,
    TEMP = 5,
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

