#[macro_use]
extern crate clap;

mod sha0;
mod sha1;
mod sha224;
mod sha256;
mod sha384;
mod sha512;

use std::fs;
use clap::{Arg, App};

fn main() {
	let matches = App::new("SHA CLI")
        .version(crate_version!())
        .author("Connor Belman")
        .about("Simple SHA hash program for the command line")
        .after_help("Available Variants: 0, 1, 224, 256, 384, 512")
        .arg(Arg::with_name("variant")
                .required(true)
                .help("desired hash variant"))
        .arg(Arg::with_name("from_file")
        	.short("f")
        	.long("file")
        	.help("Read input from file"))
        .arg(Arg::with_name("from_text")
        	.conflicts_with("from_file")
        	.short("t")
        	.long("text")
        	.help("read input from commmand line (doesn't read white space)"))
        .arg(Arg::with_name("input")
        	.required(true)
        	.help("file or text to be hashed"))
        .get_matches();
        
	let mut output = String::from("");
        let mut message = String::from("");

        if let Some(input) = matches.value_of("input") {
                if matches.is_present("from_text") {
                        message = input.to_string(); 
                }
                else if matches.is_present("from_file") {
                        message = fs::read_to_string(input).expect("could not read file");
                }
                match matches.value_of("variant").unwrap() {
                        "0" => output = sha0::generate(message),
                        "1" => output = sha1::generate(message),
                        "224" => output = sha224::generate(message),
                        "256" => output = sha256::generate(message),
                        "384" => output = sha384::generate(message),
                        "512" => output = sha512::generate(message),
                        _ => output = "Error: invalid variant".to_string()
                };
        }

	println!("{}", output);
}