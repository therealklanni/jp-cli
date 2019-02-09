#![feature(termination_trait_lib)]
#![feature(try_trait)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

use clap::{clap_app, crate_version};
use exit::Exit;
use serde_json::{from_str, Value};

#[derive(Debug)]
enum JpErr {
    FileReadError,
    EmptyFileError,
    JsonParseError,
    InvalidQuery,
    FileOpenError,
}

impl From<JpErr> for i32 {
    fn from(err: JpErr) -> Self {
        match err {
            JpErr::FileReadError => 2,
            JpErr::EmptyFileError => 5,
            JpErr::JsonParseError => 3,
            JpErr::InvalidQuery => 4,
            JpErr::FileOpenError => 1,
        }
    }
}

fn read_from_source<T: BufRead>(reader: &mut T) -> Result<Value, JpErr> {
    let mut contents = String::new();
    // file read error
    let size = reader.read_to_string(&mut contents)
        .map_err(|_| JpErr::FileReadError)?;

    // empty file error
    if size == 0 {
        return Err(JpErr::EmptyFileError);
    }

    // JsonParseError
    let json = from_str(&contents)
        .map_err(|_| JpErr::JsonParseError)?;

    Ok(json)
}

fn print_json(value: Value, options: PrintOptions) {
    let json: &Value;

    if options.pointer == "/" {
        json = &value;
    } else {
        // invalid query
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

fn main() -> Exit<JpErr> {
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
        // fileopenerror
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: {}: {}", e, filename);
                exit(1);
            }
        };
        let mut buf_reader = BufReader::new(file);

        value = read_from_source(&mut buf_reader)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        value = read_from_source(&mut handle)?;
    }

    let options = PrintOptions {
        pointer,
        pretty: matches.is_present("PRETTY"),
    };

    print_json(value, options);

    Exit::Ok
}

struct PrintOptions {
    pointer: String,
    pretty: bool,
}
