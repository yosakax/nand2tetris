use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct CodeGen {
    comp: HashMap<String, String>,
    jump: HashMap<String, String>,
    dest: HashMap<String, String>,
    symbols: HashMap<String, usize>,
}

impl CodeGen {
    pub fn new() -> Self {
        let comp_mnemonic = vec![
            "M".to_string(),
            "!M".to_string(),
            "-M".to_string(),
            "M+1".to_string(),
            "M-1".to_string(),
            "D+M".to_string(),
            "D-M".to_string(),
            "M-D".to_string(),
            "D&M".to_string(),
            "D|M".to_string(),
            // a = 0
            "0".to_string(),
            "1".to_string(),
            "-1".to_string(),
            "D".to_string(),
            "A".to_string(),
            "!D".to_string(),
            "!A".to_string(),
            "-D".to_string(),
            "-A".to_string(),
            "D+1".to_string(),
            "A+1".to_string(),
            "D-1".to_string(),
            "A-1".to_string(),
            "D+A".to_string(),
            "D-A".to_string(),
            "A-D".to_string(),
            "D&A".to_string(),
            "D|A".to_string(),
        ];
        let comp_bin = vec![
            "110000".to_string(),
            "110001".to_string(),
            "110011".to_string(),
            "110111".to_string(),
            "110010".to_string(),
            "000010".to_string(),
            "010011".to_string(),
            "000111".to_string(),
            "000000".to_string(),
            "010101".to_string(),
            // a = 0
            "101010".to_string(),
            "111111".to_string(),
            "111010".to_string(),
            "001100".to_string(),
            "110000".to_string(),
            "001101".to_string(),
            "110001".to_string(),
            "001111".to_string(),
            "110011".to_string(),
            "011111".to_string(),
            "110111".to_string(),
            "001110".to_string(),
            "110010".to_string(),
            "000010".to_string(),
            "010011".to_string(),
            "000111".to_string(),
            "000000".to_string(),
            "010101".to_string(),
        ];

        let dest_mnemonic = vec![
            "null".to_string(),
            "M".to_string(),
            "D".to_string(),
            "DM".to_string(),
            "MD".to_string(),
            "A".to_string(),
            "AM".to_string(),
            "AD".to_string(),
            "ADM".to_string(),
        ];
        let dest_bin = vec![
            "000".to_string(),
            "001".to_string(),
            "010".to_string(),
            "011".to_string(),
            "011".to_string(),
            "100".to_string(),
            "101".to_string(),
            "110".to_string(),
            "111".to_string(),
        ];

        let jump_mnemonic = vec![
            "null".to_string(),
            "JGT".to_string(),
            "JEQ".to_string(),
            "JGE".to_string(),
            "JLT".to_string(),
            "JNE".to_string(),
            "JLE".to_string(),
            "JMP".to_string(),
        ];
        let jump_bin = vec![
            "000".to_string(),
            "001".to_string(),
            "010".to_string(),
            "011".to_string(),
            "100".to_string(),
            "101".to_string(),
            "110".to_string(),
            "111".to_string(),
        ];

        let mut comp = HashMap::new();
        for i in 0..comp_mnemonic.len() {
            comp.insert(comp_mnemonic[i].to_owned(), comp_bin[i].to_owned());
        }
        let mut dest = HashMap::new();
        for i in 0..dest_mnemonic.len() {
            dest.insert(dest_mnemonic[i].to_owned(), dest_bin[i].to_owned());
        }

        let mut jump = HashMap::new();
        for i in 0..jump_mnemonic.len() {
            jump.insert(jump_mnemonic[i].to_owned(), jump_bin[i].to_owned());
        }

        let symbols = HashMap::new();

        let code = CodeGen {
            comp,
            dest,
            jump,
            symbols,
        };
        code
    }

    pub fn gen_comp(&self, code: &String) -> String {
        // eprintln!("{}", code);
        self.comp.get(code).unwrap().to_owned()
    }
    pub fn gen_dest(&self, code: &String) -> String {
        // eprintln!("{}", code);
        self.dest.get(code).unwrap().to_owned()
    }
    pub fn gen_jump(&self, code: &String) -> String {
        // eprintln!("{}", code);
        self.jump.get(code).unwrap().to_owned()
    }
    pub fn gen_abit(&self, code: &String) -> String {
        return match code.contains(&"M") {
            true => "1".to_string(),
            false => "0".to_string(),
        };
    }
}

