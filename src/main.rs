use std::{env,fs::File,process::exit};
use colored::Colorize;

fn main() {
    let args: Vec<String> = vec![env::args().collect()];

    match args { //this matches from the start if user hasn't inputed anything and just ran the program, cool colors tho
        _ => {println!("Please input parameters like this, {} {} {}", "rustcompressor".yellow(), "input.xyz".blue(), "output.xyz".red()); exit(1);}
    }
}
