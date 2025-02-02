use std::env;
use std::fmt;
use std::process;

fn help() {
    println!("Supported codecs:");
    println!("{}", espa_codecs::list());
}

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

fn exit_with_error(message: impl fmt::Display) -> ! {
    eprintln!("{}", message);
    help();
    process::exit(EXIT_FAILURE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        exit_with_error("Usage: <codec> <operation> <input> <output>")
    }

    let codec = &args[1];
    let operation = &args[2];
    let input = &args[3];
    let output = &args[4];
    println!("Selected options: {codec} {operation} {input} {output}");

    match codec.as_str() {
        "lz77" => {
            // NYI
        }
        _ => {
            exit_with_error(format!("Unsupported codec: {codec}"));
        }
    }

    match operation.as_str() {
        "compress" => {
            // NYI
        }
        "decompress" => {
            //  NYI
        }
        _ => exit_with_error(format!("Unsupported operation: {operation}")),
    }

    process::exit(EXIT_SUCCESS);
}
