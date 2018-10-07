#![feature(custom_attribute)]

mod project;

#[macro_use]
extern crate clap;
extern crate colored;
extern crate indicatif;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;

use clap::App;
use colored::*;
//use indicatif::{ProgressBar, ProgressStyle};
use project::{parse_plan, Project};
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
}

fn run(path: String) -> Result<String, Box<Error>> {
    let p = parse_plan(&path)?;
    let v: Project = serde_yaml::from_str(&p)?;
    println!(
        "\u{1F3E0}{:>16}\n{:>8}{:2}\n",
        "Executing Plan".purple(),
        "File:".purple(),
        &path
    );
    let z = v.resolve_requirements()?;
    println!("\n{}\n", z.green());
    return v.execute();
}

//#[cfg(test)]
//mod main_tests;
