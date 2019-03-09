#![feature(termination_trait_lib)]
#![feature(try_trait)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{clap_app, crate_version};
use exit::Exit;
use serde_json::{from_str, to_value, Value};

#[derive(Debug)]
enum JpErr {
    FileReadError,
    EmptyFileError,
    JsonParseError,
    InvalidQuery(String),
    FileOpenError(String),
}

impl From<JpErr> for i32 {
    fn from(err: JpErr) -> Self {
        match err {
            JpErr::FileReadError => 2,
            JpErr::EmptyFileError => 5,
            JpErr::JsonParseError => 3,
            JpErr::InvalidQuery(_) => 4,
            JpErr::FileOpenError(_) => 1,
        }
    }
}

fn read_from_source<T: BufRead>(reader: &mut T) -> Result<Value, JpErr> {
    let mut contents = String::new();
    let size = reader.read_to_string(&mut contents)
        .map_err(|_| JpErr::FileReadError)?;

    if size == 0 {
        return Err(JpErr::EmptyFileError);
    }

    let json = from_str(&contents)
        .map_err(|_| JpErr::JsonParseError)?;

    Ok(json)
}

fn print_json(value: Value, options: PrintOptions) -> Result<(), JpErr> {
    let pointer_as_query: String = options.pointer[1..].replace('/', ".");

    let mut json = if options.pointer == "/" {
        &value
    } else {
        value.pointer(&options.pointer)
            .ok_or(JpErr::InvalidQuery(pointer_as_query.clone()))?
    };

    let json_value: Value;

    json = if options.keys {
        let json_keys = match json {
            Value::Object(ref v) => v.keys(),
            _ => {
                eprintln!("Error: cannot print keys of a non-object: {}", pointer_as_query);
                std::process::exit(6);
            }
        };

        let keys_as_vec: Vec<String> = json_keys.cloned().collect();
        json_value = to_value(keys_as_vec).unwrap();
        &json_value
    } else {
        &json
    };

    if options.pretty {
        println!("{:#}", json);
    } else {
        println!("{}", json);
    }

    Ok(())
}

fn main() -> Exit<JpErr> {
    let matches = clap_app!(jp =>
        (version: crate_version!())
        (about: "JSON Probe (http://github.com/therealklanni/jp-cli)")
        (@arg FILE: -f --file +takes_value "JSON file to probe")
        (@arg PRETTY: -P --pretty "Prints pretty format for humans")
        (@arg KEYS: -k --keys "Prints the keys of the object")
        (@arg PATTERN: "Query pattern")
    )
    .get_matches();

    let mut pointer = String::from("/");
    let value: Value;

    if matches.is_present("PATTERN") {
        let pattern = matches.value_of("PATTERN").unwrap();
        let prefixed = format!("{}{}", "/", pattern);
        pointer = prefixed.replace(".", "/");
    }

    if matches.is_present("FILE") {
        let filename = matches.value_of("FILE").unwrap();
        let file = File::open(filename)
            .map_err(|_| JpErr::FileOpenError(filename.to_string()))?;
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
        keys: matches.is_present("KEYS"),
    };

    print_json(value, options)?;

    Exit::Ok
}

struct PrintOptions {
    pointer: String,
    pretty: bool,
    keys: bool,
}
