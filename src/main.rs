#![feature(custom_attribute)]

mod parser;

#[macro_use]
extern crate clap;
extern crate colored;
extern crate indicatif;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;

use clap::App;
use colored::*;
//use indicatif::{ProgressBar, ProgressStyle};
use parser::{parse_plan, Project};
use std::error::Error;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(path) = matches.value_of("path") {
        match run(path.to_string()) {
            Ok(s) => println!("{ }", s),
            Err(e) => println!("Error: { }", e.to_string()),
        }
    }

    //let args = matches.value_of("args");
    //match args {
    //    Some(a) => println!("arguments provided{ }", a),
    //    None => println!("no arguments were provided"),
    //}
}

fn run(path: String) -> Result<String, Box<Error>> {
    let p = parse_plan(&path)?;
    let v: Project = serde_yaml::from_str(&p)?;
    println!(
        "\u{1F3E0}  {}\n   {} {}\n",
        "Executing Plan".purple(),
        "File:".purple(),
        &path
    );
    return v.execute();
}

//#[cfg(test)]
//mod main_tests;
