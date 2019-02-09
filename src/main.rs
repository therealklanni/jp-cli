use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

#[macro_use]
extern crate clap;

extern crate serde_json;
use serde_json::{from_str, Value};

fn read_from_source<T: BufRead>(reader: &mut T) -> Value {
    let mut contents = String::new();
    let size = match reader.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    };

    if size == 0 {
        exit(0);
    }

    match from_str(&contents) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(3);
        }
    }
}

fn print_json(value: Value, options: PrintOptions) {
    let json: &Value;

    if options.pointer == "/" {
        json = &value;
    } else {
        json = match value.pointer(&options.pointer) {
            None => {
                eprintln!("Invalid query: {}", options.pointer[1..].replace('/', "."));
                exit(4);
            }
            value => value.unwrap(),
        }
    }

    if options.pretty {
        println!("{:#}", json);
    } else {
        println!("{}", json);
    }
}

fn main() {
    let matches = clap_app!(jp =>
        (version: crate_version!())
        (about: "JSON Probe (http://github.com/therealklanni/jp-cli)")
        (@arg FILE: -f --file +takes_value "JSON file to probe")
        (@arg PRETTY: -P --pretty "Prints pretty format for humans")
        (@arg PATTERN: "Query pattern")
    )
    .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap_or("");
    let prefixed = format!("{}{}", "/", pattern);
    let pointer = prefixed.replace(".", "/");
    let value: Value;

    if matches.is_present("FILE") {
        let filename = matches.value_of("FILE").unwrap();
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: {}: {}", e, filename);
                exit(1);
            }
        };
        let mut buf_reader = BufReader::new(file);

        value = read_from_source(&mut buf_reader);
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        value = read_from_source(&mut handle);
    }

    let options = PrintOptions {
        pointer,
        pretty: matches.is_present("PRETTY"),
    };

    print_json(value, options);
}

struct PrintOptions {
    pointer: String,
    pretty: bool,
}
