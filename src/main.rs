// Std imports
use std::{env, fs};

// Local module
mod core_module;
use core_module::interpreter::Interpreter;

// Colored
use colored::*;

// Huff imports
use huff_utils::prelude::EVMVersion;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut debug_level: Option<u8> = None;
    let mut evm_version: EVMVersion = EVMVersion::default();

    if args.contains(&"--debug".to_string()) {
        debug_level = Some(255);
    }

    if args.contains(&"--evm".to_string()) {
        // fetch the string provided version
        let version = args[args.iter().position(|x| x == "--evm").unwrap() + 1].clone();
        evm_version = EVMVersion::from(version);
    }

    // The bytecode path is not an argument, but the last argument
    if args.len() > 1 {
        // Read the huff file
        let file_path = &args[args.len() - 1];

        let mut interpreter = Interpreter::new(
            file_path.as_str(),
            evm_version,
            debug_level
        );

        interpreter.interpret();

    } else {
        print_help();
        return Ok(());
    }

    Ok(())
}

fn print_help() {
    println!("Generate the stack comment in a Huff file.");
    println!("\nUsage: {} <{}>", "huffr".green(), "file_path".cyan());
    println!(
        "       {} is a path to a {}.\n",
        "file_path".cyan(),
        "Huff file".yellow()
    );
}
