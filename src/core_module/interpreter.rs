use std::sync::Arc;

// Huff imports
use huff_core::Compiler;

// evm-rs-emulator imports
use evm_rs_emulator::{
    debug::vec_to_hex_string,
    Runner,
};

// Colored
use colored::*;
use huff_lexer::Lexer;
use huff_utils::{
    prelude::EVMVersion,
    token::{Token, TokenKind},
};

pub struct Interpreter {
    pub file_path: String,
    pub huff_source: String,
    pub lines: Vec<Vec<Token>>,
    pub tokens: Vec<Token>,
    pub runner: Runner,
    pub huff_output: Vec<String>,
    pub evm_version: EVMVersion,
}

impl Interpreter {
    /// Creates a new instance of the interpreter with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The huff source code to be interpreted.
    /// * `evm_version` - The EVM version to be used.
    /// * `debug_level` - The debug level to be used.
    /// * `base_path` - The base path to be used.
    ///
    /// # Returns
    ///
    /// A new instance of the interpreter.
    pub fn new(file_path: &str, evm_version: EVMVersion, debug_level: Option<u8>) -> Self {
        let runner = Runner::_default(debug_level.unwrap_or(0));

        let mut instance = Self {
            file_path: String::from(file_path),
            runner,
            tokens: Vec::new(),
            huff_output: Vec::new(),
            huff_source: String::new(),
            lines: Vec::new(),
            evm_version,
        };

        instance.extract_lines();

        // split the huff source code into lines and store it in huff_output
        instance.huff_output = instance.huff_source.split("\n").map(|x| x.to_string()).collect();

        println!("{} {:?}", "Tokens".green(), instance.huff_output);

        instance
    }

    /// Creates a new instance of the interpreter with default parameters.
    ///
    /// # Arguments
    ///
    /// debug_level - The debug level to be used.
    ///
    /// # Returns
    ///
    /// A new instance of the interpreter.
    pub fn _default(&mut self, debug_level: Option<u8>) -> Self {
        Self::new("", EVMVersion::default(), debug_level)
    }

    /// Interprets the huff source code and generates the output.
    pub fn interpret(&mut self) {
        let compiler = Compiler::new(
            &self.evm_version,
            Arc::new(vec![self.file_path.to_string()]),
            None,
            None,
            None,
            None,
            None,
            false,
            false,
        );

        // Compile the source code and extract the macros with their statements
        let res = compiler.grab_contracts();

        let mut i = 0;
        for line in self.lines.clone().iter() {
            println!("{} {}", "Line".green(), i);
            self.process_line(line);

            i += 1;
        }

        // for each macros in the vec, print all the statements
        for macro_ in res.unwrap()[0].macros.iter() {
            // Ici, resolved_statements contient tous les statements de macro_ résolus et concaténés
            println!("{} {}", "Resolved Macro".green(), macro_.name);
            for statement in macro_.statements.iter() {
                println!("{:?}", statement);
            }
        }
    }

    fn extract_lines(&mut self) {
        // Extract the tokens from the source code
        self.huff_source = std::fs::read_to_string(self.file_path.as_str()).unwrap();
        let lexer = Lexer::new(self.huff_source.as_str());
        self.tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<Token>>();

        let mut lines = Vec::new();
        let mut line = Vec::new();
        for token in self.tokens.iter() {
            let string_token = self.huff_source[token.span.start..token.span.end].to_string();
            if string_token.contains("\n") {
                lines.push(line);
                line = Vec::new();
            } else if (matches!(token.kind, TokenKind::Opcode(_))
                || matches!(token.kind, TokenKind::Label(_))
                || matches!(token.kind, TokenKind::BuiltinFunction(_))
                || matches!(token.kind, TokenKind::Literal(_))
                || matches!(token.kind, TokenKind::Num(_))
                || matches!(token.kind, TokenKind::Ident(_))
                || matches!(token.kind, TokenKind::Macro))
            {
                line.push(token.clone());
            }
        }
        self.lines = lines;
    }

    fn process_line(&mut self, line: &Vec<Token>) {
        for token in line.iter() {
            match token.kind {
                TokenKind::Opcode(_) => {
                    // Convert token kind to string
                    let opcode = token.kind.to_string();

                    // Convert the opcode hex string into a u8
                    let opcode = u8::from_str_radix(opcode.as_str(), 16).unwrap();

                    self.runner
                        .bytecode
                        .push(opcode);
                }
                TokenKind::Label(_) => {}
                TokenKind::BuiltinFunction(_) => {}
                TokenKind::Literal(_) => {
                    let push = Self::process_push(&token.kind.to_string());

                    // Push the opcode
                    self.runner.bytecode.push(push.0);

                    // Push the bytes
                    if push.0 != 0x5f {
                        for byte in push.1 {
                            self.runner.bytecode.push(byte);
                        }
                    }
                }
                TokenKind::Num(_) => {}
                TokenKind::Ident(_) => {}
                TokenKind::Macro => {
                    self.setup_new_macro();
                    return;
                }
                _ => {}
            }
        }
        println!("{}", vec_to_hex_string(self.runner.bytecode.to_owned()));

        let _ = self.runner.interpret(
            self.runner.bytecode.to_owned(),
            self.runner.debug_level,
            true,
        );

        // print stack
        println!("{} {:?}", "Stack".green(), self.runner.stack);
        println!("\n\n\n\n");

        // Write the stack to the output
        if line.len() == 0 {return};

        // Add stack at the end of the line
        let line_string= self.huff_source[line[0].span.start..line[line.len() - 1].span.end+1].to_string();
        for i in 0..self.huff_output.len() {
            println!("{} {:?}", line_string, line);

            if self.huff_output[i].contains(&line_string) {
                self.huff_output[i] = format!(
                    "{}{}",
                    self.huff_output[i].clone(),
                    format!(" // {:?}", self.runner.stack)
                );
            }
        }
    }

    fn process_push(litteral: &str) -> (u8, Vec<u8>) {
        let mut stripped = litteral.trim_start_matches('0');
        if stripped.is_empty() {
            stripped = "0";
        }

        // Make stripped odd length
        let ood_stripped = if stripped.len() % 2 != 0 {
            format!("0{}", stripped)
        } else {
            stripped.to_string()
        };

        // Convert the string into a vector of bytes
        let hex_val = hex::decode(ood_stripped).unwrap();

        // Return push0
        if stripped == "0" {
            return (0x5f, hex_val);
        }

        // Calculate the required length in bytes
        let len = (stripped.len() + 1) / 2;

        if len > 32 {}

        // Convert the length into the opcode
        ((0x60 + len - 1) as u8, hex_val)
    }

    fn setup_new_macro(&mut self) {
        self.runner = Runner::_default(self.runner.debug_level.unwrap());
    }
}
