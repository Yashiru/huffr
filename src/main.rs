// Std imports
use std::{env, fs, vec};

// Colored
use colored::*;

// Huff imports
use huff_lexer::Lexer;
use huff_utils::prelude::{ToIRBytecode, EVMVersion};
use huff_utils::token::*;
use huff_parser::Parser;

// evm-rs-emulator imports
use evm_rs_emulator::Runner;
use evm_rs_emulator::bytes::_hex_string_to_bytes;

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
        // Remove file name from path
        let path_elem = file_path.split("/").collect::<Vec<&str>>();
        let base_path = path_elem[..path_elem.len() - 1].join("/");
        let base_path = format!("./{}", base_path);

        let huff_source =
            fs::read_to_string(file_path).expect("Something went wrong reading the file");
        // let mut bytecode = vec![];

        // Colored debug
        println!("{}: \n{}\n", "Huff source".blue(), huff_source);

        // Create a lexer
        let lexer = Lexer::new(huff_source.as_str());

        // Create a parser
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, Some(base_path));

        // Color debug
        println!("{}: \n{:?}\n", "Tokens".blue(), parser.tokens);

        // Parse into AST
        let unwrapped_contract = parser.parse().unwrap();

        // Color debug
        println!("{}: \n {:?}\n", "AST".blue(), unwrapped_contract);

        // Color debug
        println!("{}", "Statement".blue());

        // print all statements of all macros
        for macro_ in unwrapped_contract.macros.iter() {
            for statement in macro_.statements.iter() {
                println!("{:?}", statement);
            }
        }
        println!();

        parser.reset();
        let res = parser.parse_imports();
        
        // Color debug
        println!("{}: \n{:?}\n", "Imports".blue(), res);

        let ir_bytecode = unwrapped_contract.macros[0].to_irbytecode(&evm_version);

        // Color debug
        println!("{}: \n{:?}\n", "Bytecode".blue(), ir_bytecode.unwrap().0);

        // println!("{:?}", unwrapped_contract.macros[0]);

    } else {
        print_help();
        return Ok(());
    }

    Ok(())
}

fn create_runner(debug_level: Option<u8>) -> Runner {
    let caller = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xc4, 0x11, 0xe8,
    ];
    let origin: Option<[u8; 20]> = None;
    let address: Option<[u8; 20]> = Some([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xc4, 0x11, 0xee,
    ]);
    let value: Option<[u8; 32]> = None;
    let data: Option<Vec<u8>> = None;

    // Create a new interpreter
    let mut runner = Runner::new(caller, origin, address, value, data, None);
    runner.debug_level = debug_level;

    runner
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
