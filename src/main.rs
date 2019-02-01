use std::fs::File;
use std::io::{self, Read};
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_json;
use serde_json::{from_str, Value};

fn main() {
    let matches = clap_app!(jp =>
        (version: "0.0.1")
        (about: "Simple JSON parser/inspector")
        (@arg FILE: -f --file +takes_value "JSON file to parse")
        (@arg PATTERN: "Query pattern")
    )
    .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap_or("");
    let prefixed: String = format!("{}{}", "/", pattern);
    let pointer: &str = &prefixed.replace(".", "/");

    if matches.is_present("FILE") {
        let filename = matches.value_of("FILE").unwrap();
        let mut contents = String::new();
        let mut file = File::open(filename).expect("file not found");

        file.read_to_string(&mut contents)
            .expect("I/O error reading file");

        let value: Value = from_str(&contents).unwrap();

        if matches.is_present("PATTERN") {
            // println!("{:?}", pointer);
            println!("{:#}", value.pointer(pointer).unwrap_or(&json!("")));
        } else {
            println!("{:#}", value);
        }
    } else {
        println!("No file option specified");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle
            .read_to_string(&mut buffer)
            .expect("I/O error reading buffer");

        let value: Value = from_str(&buffer).unwrap();

        if matches.is_present("PATTERN") {
            // println!("{:?}", pointer);
            println!("{:#}", value.pointer(pointer).unwrap_or(&json!("")));
        } else {
            println!("{:#}", value);
        }
    }
}
