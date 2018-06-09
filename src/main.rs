use std::fs::File;
use std::io::prelude::*;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_json;
use serde_json::{Value, from_str};

fn main() {
  let matches = clap_app!(jp =>
    (version: "0.1.0")
    (author: "Kevin Lanni <therealklanni@gmail.com>")
    (about: "Simple JSON parser/inspector")
    (@arg FILE: -f --file +takes_value "JSON file to parse")
    (@arg PATTERN: "Query pattern")
  ).get_matches();

  let filename = matches.value_of("FILE").unwrap();
  let pattern = matches.value_of("PATTERN").unwrap_or("");
  let mut f = File::open(filename).expect("file not found");
  let mut contents = String::new();

  f.read_to_string(&mut contents).expect("error reading file");

  let value: Value = from_str(&contents).unwrap();

  let prefix: String = "/".to_owned() + pattern;

  let pointer: &str = &prefix.replace(".", "/");

  println!("{:#}", contents);
  println!("{:?}", pointer);
  println!("{:#}", value.pointer(pointer).unwrap_or(&json!("")));
}
