use colored::*;
use evm_rs_emulator::Runner;
use evm_rs_emulator::bytes::_hex_string_to_bytes;
use huff_lexer::Lexer;
use huff_utils::{
    token::{self, *},
};
use std::{env, fs, vec};

// Colored output

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut debug_level: Option<u8> = None;

    if args.contains(&"--debug".to_string()) {
        debug_level = Some(255);
    }

    // The bytecode path is not an argument, but the last argument
    if args.len() > 1 {
        // Read the huff file
        let file_path = &args[args.len() - 1];
        let huff_source =
            fs::read_to_string(file_path).expect("Something went wrong reading the file");
        let mut bytecode = vec![];

        // Create a lexer
        let mut lexer = Lexer::new(huff_source.as_str());

        // Create a runner
        let mut runner = create_runner(debug_level);

        loop {
            let token = lexer.next().unwrap().unwrap();
            if token.kind.to_string().len() == 64 {
                bytecode.push(0x7f);
                // For each byte in the hex string push it to the bytecode
                for byte in _hex_string_to_bytes(token.kind.to_string().as_str()) {
                    bytecode.push(byte);
                }

                runner.bytecode = bytecode.clone();
                let res = runner.interpret_op_code(runner.bytecode[runner.pc as usize]);
                if res.is_err() {
                    println!("Error: {}", format!("{:?}", res).red());
                    return Ok(());
                }
            }

            if token.kind.to_string().len() == 2 {
                let hex_string = token.kind.to_string();
                bytecode.push(u8::from_str_radix(&hex_string, 16).unwrap());

                runner.bytecode = bytecode.clone();
                let res = runner.interpret_op_code(runner.bytecode[runner.pc as usize]);
                if res.is_err() {
                    println!("Error: {}", format!("{:?}", res).red());
                    return Ok(());
                }
            }

            if token.kind == TokenKind::Whitespace {
                if lexer.peek_n_chars(0).contains("\n") {
                    println!("=== New Line ===");
                }
            }

            if token.kind == TokenKind::Eof {
                break;
            }
        }
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
