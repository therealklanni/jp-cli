use std::fs::File;
use std::io::{self, Read};
use std::process::exit;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_json;
use serde_json::{from_str, Value};

fn main() {
    let matches = clap_app!(jp =>
        (version: crate_version!())
        (about: "JSON Probe (http://github.com/therealklanni/jp)")
        (@arg FILE: -f --file +takes_value "JSON file to probe")
        (@arg PATTERN: "Query pattern")
    )
    .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap_or("");
    let prefixed: String = format!("{}{}", "/", pattern);
    let pointer: &str = &prefixed.replace(".", "/");

    if matches.is_present("FILE") {
        let filename = matches.value_of("FILE").unwrap();
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                println!("Error: {}: {}", e, filename);
                exit(1);
            }
        };

        let mut contents = String::new();
        let size = match file.read_to_string(&mut contents) {
            Ok(size) => size,
            Err(e) => {
                println!("Error: {}", e);
                exit(2);
            }
        };

        if size == 0 {
            exit(0);
        }

        let value: Value = from_str(&contents).unwrap();

        if matches.is_present("PATTERN") {
            println!("{:#}", value.pointer(pointer).unwrap_or(&json!("")));
        } else {
            println!("{:#}", value);
        }
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = String::new();

        let size = match handle.read_to_string(&mut buffer) {
            Ok(size) => size,
            Err(e) => {
                println!("Error: {}", e);
                exit(3);
            }
        };

        if size == 0 {
            exit(0);
        }

        let value: Value = from_str(&buffer).unwrap();

        if matches.is_present("PATTERN") {
            println!("{:#}", value.pointer(pointer).unwrap_or(&json!("")));
        } else {
            println!("{:#}", value);
        }
    }
}
