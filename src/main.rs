use std::{env,fs::{self,File},process::exit,io::{Read,Write}};
use colored::Colorize;
use deflate::deflate_bytes;
fn main() {
    println!("Rust Compressor, only uses {} compression type, only works in current dir", "deflate".cyan());
    let args: Vec<String> = env::args().collect();
    match args.len() {
        
        1 => {println!("Please input parameters like this, {} {} {}", "rustcompressor".yellow(), "input.xyz".blue(), "output.xyz".red());},
        2 => {println!("Please input parameters like this, {} {} {}", "rustcompressor".yellow(), "input.xyz".blue(), "output.xyz".red());},
        3 => {println!("Compressing {} into {}", &args[1].blue(), &args[2].red()); check_files(args);},
        _ => todo!()
    };
}

fn check_files(args: Vec<String>) {
    let input_file: String  = args[1].to_string();
    let output_file: String = args[2].to_string();

    if let Ok(current_dir) = env::current_dir() {
        match fs::read_dir(current_dir) {
            Ok(dir) => {
                 // first check if output file exists and stop execution

                for entry in &mut dir.filter_map(Result::ok) {

                    if entry.file_name().to_str() == Some(&output_file) {
                        panic!("{}", "Output file already exists, please delete or choose a different name".red());
                    } 

                    if entry.file_name().to_str() == Some(&input_file) {
                        println!("<25%===="); readandcompress(&input_file, &output_file)

                    } 
                } }
            Err(err) => eprintln!("Error reading directory: {}", err),
        }
    } else {
        eprintln!("Error getting current working directory");
    }
}

fn readandcompress(input_file: &String, output_file: &String) { 
    let output_file:String = output_file.to_string();
    if let Ok(mut input_file) = File::open(input_file) {
        // Read the contents of the file into a Vec<u8>
        print!("50%");
        let mut file_data = Vec::new();
        if let Ok(_) = input_file.read_to_end(&mut file_data) {
            // Now you have the file data in the `file_data` vector
            let compressed_data = deflate_bytes(&file_data);
            print!("====75%")
            if let Ok(mut output_file) = File::create(output_file + ".deflate") {
                
                if let Err(err) = output_file.write_all(&compressed_data) {
                    eprintln!("Error writing to output file: {}", err);
                } else {
                    print!("====100%>"); exit(1);}
           
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
    
