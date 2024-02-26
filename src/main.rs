use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{Arg, ArgAction, command, value_parser};

use bfuck::interpret::interpret;
use bfuck::code::process_code;
use bfuck::jit::jit;

fn main() {
    let argv = command!()
        .next_line_help(true)
        .arg(Arg::new("src_file")
            .value_name("SRC_FILE")
            .help("The Brainfuck file.")
            .required(true)
            .value_parser(value_parser!(PathBuf)))
        .arg(Arg::new("interpret")
            .short('i')
            .long("interpret")
            .action(ArgAction::SetTrue)
            .help("Interpret Brainfuck code. [default]")
            .conflicts_with_all(["jit", "compile"])
            .required(false))
        .arg(Arg::new("jit")
            .short('j')
            .long("jit")
            .action(ArgAction::SetTrue)
            .help("Execute code using Just-in-time (JIT) compilation.")
            .conflicts_with_all(["interpret", "compile"])
            .required(false))
        .arg(Arg::new("compile")
            .short('c')
            .long("compile")
            .action(ArgAction::SetTrue)
            .help("Compile code to executable.")
            .conflicts_with_all(["interpret", "jit"])
            .required(false))
        .arg(Arg::new("dst_file")
            .value_name("DST_FILE")
            .help("The compiled file.")
            .requires("compile")
            .conflicts_with_all(["interpret", "jit"])
            .value_parser(value_parser!(PathBuf)))
        .get_matches();

    let src_file = Path::new(argv.get_one::<PathBuf>("src_file").unwrap().to_str().unwrap());

    let mut interpret_flag: bool = argv.get_flag("interpret");
    let jit_flag: bool = argv.get_flag("jit");
    let compile_flag: bool = argv.get_flag("compile");
    
    if !(interpret_flag || jit_flag || compile_flag) {
        interpret_flag = true;
    }
    
    let mut _dst_file = Path::new("");
    if compile_flag {
        _dst_file = Path::new(argv.get_one::<PathBuf>("dst_file").unwrap().to_str().unwrap());
    }

    let code = match fs::read_to_string(src_file) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading the Brainfuck file: {}", err);
            exit(1);
        },
    };
    let token_stream = process_code(&code).unwrap();

    // todo
    if interpret_flag {
        interpret(token_stream);
    } else if jit_flag {
        if let Err(err) = jit(token_stream) {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
