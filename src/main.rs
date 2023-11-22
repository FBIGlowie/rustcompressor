use colored::Colorize;
use std::{
    env,
    fs::{self, File},
    io::prelude::*,
    path::Path,
    process::exit,
};

use flate2::write::GzEncoder;
use flate2::Compression;

fn main() {
    println!(
        "Rust Compressor, only uses {} compression type, only works in current dir",
        "deflate".cyan()
    );
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!(
                "Please input parameters like this, {} {} {}",
                "rustcompressor".yellow(),
                "input.xyz".blue(),
                "output.xyz".red()
            );
        }
        2 => {
            println!(
                "Please input parameters like this, {} {} {}",
                "rustcompressor".yellow(),
                "input.xyz".blue(),
                "output.xyz".red()
            );
        }
        3 => {
            println!(
                "Compressing {} into {}{}",
                &args[1].blue(),
                &args[2].red(),
                ".gz".red()
            );
            check_files(args);
        }
        _ => todo!(),
    }
}

fn check_files(args: Vec<String>) {
    let input_file: String = args[1].to_string();
    let output_file: String = args[2].to_string();
    let output_file_gz: String = format!("{}.gz", &output_file);

    if Path::new(&output_file).exists() || Path::new(&output_file_gz).exists() {
        panic!(
            "{}",
            "Output file already exists, please delete or choose a different name".red()
        );
    } else if Path::new(&input_file).exists() {
        println!("<====25%====");
        readandcompress(&input_file, &output_file);
    } else {
        panic!("{}", "Input file doesn't exist in current dir".red());
    }
}

fn readandcompress(input_file: &String, output_file: &String) {
    let output_file: String = format!("{}.gz", output_file);
    if let Ok(mut input_file) = File::open(input_file) {
        // Read the contents of the file into a Vec<u8>
        print!("50%");
        let mut file_data = Vec::new();
        if let Ok(_) = input_file.read_to_end(&mut file_data) {
            // Now you have the file data in the `file_data` vector
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(file_data.as_mut_slice()).unwrap();
            let compressed_data = e.finish().unwrap();
            print!("====75%");
            if let Ok(mut output_file) = File::create(output_file) {
                if let Err(err) = output_file.write_all(&compressed_data) {
                    eprintln!("Error writing to output file: {}", err);
                } else {
                    print!("====100%>");
                    println!("DONE!");
                    exit(1);
                }
            } else {
                eprintln!("Error creating output file");
            }
        } else {
            eprintln!("Error reading input file");
        }
    } else {
        eprintln!("Error opening input file");
    }
}

/*  */
