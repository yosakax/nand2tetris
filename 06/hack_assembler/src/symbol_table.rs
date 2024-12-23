use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let symbols_string = vec![
            "R0".to_string(),
            "R1".to_string(),
            "R2".to_string(),
            "R3".to_string(),
            "R4".to_string(),
            "R5".to_string(),
            "R6".to_string(),
            "R7".to_string(),
            "R8".to_string(),
            "R9".to_string(),
            "R10".to_string(),
            "R11".to_string(),
            "R12".to_string(),
            "R13".to_string(),
            "R14".to_string(),
            "R15".to_string(),
            "SP".to_string(),
            "LCL".to_string(),
            "ARG".to_string(),
            "THIS".to_string(),
            "THAT".to_string(),
            "SCREEN".to_string(),
            "KBD".to_string(),
        ];
        let symbols_bin: Vec<usize> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 16384, 24576,
        ];
        let mut symbols: HashMap<String, usize> = HashMap::new();
        for i in 0..symbols_string.len() {
            symbols.insert(symbols_string[i].to_owned(), symbols_bin[i].to_owned());
        }

        SymbolTable { symbols }
    }

    pub fn add_entry(&mut self, symbol_name: String, address: usize) -> () {
        eprintln!("{}, {}", symbol_name, address);
        self.symbols.insert(symbol_name, address);
    }

    pub fn contains(&self, symbol_name: &String) -> bool {
        self.symbols.contains_key(symbol_name)
    }

    pub fn get_address(&mut self, symbol_name: &String) -> usize {
        eprintln!("{}", symbol_name);
        self.symbols.get(symbol_name).unwrap().to_owned()
    }
}

