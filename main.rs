use std::env;
use std::fmt;
use std::process;

fn help() {
    println!("Supported codecs: {}", espa_codecs::list());
    println!("Operations: compress decompress");
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

    let compressor_op = espa_codecs::noop_identity;
    let decompressor_op = espa_codecs::noop_identity;
    match codec.as_str() {
        "lz77" => {
            // NYI
        }
        "lz78" => {
            // NYI
        }
        "lzw" => {
            // NYI
        }
        _ => {
            exit_with_error(format!("Unsupported codec: {codec}"));
        }
    }

    match operation.as_str() {
        "compress" => {
            let decompressed = espa_bitops::read_file(input).unwrap_or_else(|why| {
                exit_with_error(format!(
                    "Failed to read file: {input} - {}",
                    why.to_string()
                ));
            });
            println!("Read {} bytes from {input}", decompressed.len());
            let compressed = compressor_op(&decompressed);
            println!("Writing {} bytes to {output}", compressed.len());
            espa_bitops::write_file(output, &compressed).expect("Failed to write file: {output}");
        }
        "decompress" => {
            let compressed = espa_bitops::read_file(input).unwrap_or_else(|why| {
                exit_with_error(format!(
                    "Failed to read file: {input} - {}",
                    why.to_string()
                ));
            });
            println!("Read {} bytes from {input}", compressed.len());
            let decompressed = decompressor_op(&compressed);
            println!("Writing {} bytes to {output}", decompressed.len());
            espa_bitops::write_file(output, &decompressed).expect("Failed to write file: {output}");
        }
        _ => exit_with_error(format!("Unsupported operation: {operation}")),
    }

    process::exit(EXIT_SUCCESS);
}
