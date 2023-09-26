use std::sync::Arc;

// Huff imports
use huff_core::Compiler;

// evm-rs-emulator imports
use evm_rs_emulator::Runner;

// Colored
use colored::*;
use huff_utils::prelude::EVMVersion;

pub struct Interpreter<'a> {
    pub file_path: &'a str,
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
    pub fn new(
        file_path: &'a str,
        evm_version: EVMVersion,
        debug_level: Option<u8>
    ) -> Self {
        let runner = Runner::_default(debug_level.unwrap());

        Self {
            file_path,
            runner,
            huff_output: String::new(),
            evm_version,
        }
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
            Arc::new(vec![
                self.file_path.to_string()
            ]),
            None,
            None,
            None,
            None,
            None,
            false,
            false,
        );

        let res = compiler.grab_contracts();

        // for each macros in the vec, print all the statements
        for macro_ in res.unwrap()[0].macros.iter() {
            for statement in macro_.statements.iter() {
                println!("{:?}", statement);
            }
        }
    }
}
