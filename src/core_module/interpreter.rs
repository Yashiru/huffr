use std::sync::Arc;

// Huff imports
use huff_core::Compiler;

// evm-rs-emulator imports
use evm_rs_emulator::Runner;

// Colored
use colored::*;
use huff_lexer::Lexer;
use huff_utils::{
    prelude::EVMVersion,
    token::{Token, TokenKind},
};

pub struct Interpreter<'a> {
    pub file_path: &'a str,
    pub huff_source: String,
    pub lines: Vec<Vec<Token>>,
    pub tokens: Vec<Token>,
    pub runner: Runner,
    pub huff_output: String,
    pub evm_version: EVMVersion,
}

impl<'a> Interpreter<'a> {
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
    pub fn new(file_path: &'a str, evm_version: EVMVersion, debug_level: Option<u8>) -> Self {
        let runner = Runner::_default(debug_level.unwrap());

        let mut instance = Self {
            file_path,
            runner,
            tokens: Vec::new(),
            huff_output: String::new(),
            huff_source: String::new(),
            lines: Vec::new(),
            evm_version,
        };

        instance.compute_lines();

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
        for line in self.lines.iter() {
            println!("{} {}", "Line".green(), i);
            for token in line.iter() {
                println!("{:?}", token)
            }

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

    fn compute_lines(&mut self) {
        // Extract the tokens from the source code
        self.huff_source = std::fs::read_to_string(self.file_path).unwrap();
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
}
