extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

// models
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub os: OS,
    pub apps: Vec<App>,
    pub plan: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OS {
    pub name: String,
    pub flavor: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
    pub package: String,
    pub manager: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Env {
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub plan: Vec<String>,
}

// functions
pub fn parse_plan(path: &String) -> Result<String, Box<Error>> {
    let mut cf = File::open("downloader.yml").expect(&format!("{} file was not found", path));

    let mut contents = String::new();
    cf.read_to_string(&mut contents)
        .expect("There was and issue reading the continuum.yaml file.");
    Ok(contents)
}

impl Project {
    pub fn execute(&self) -> Result<String, Box<Error>> {
        let s = format!("{}\n", "plan executed successfully".green());
        for cmdline in &self.plan {
            let cmdargs: Vec<&str> = cmdline.split(' ').collect();
            print!("\u{1F36C}  ");
            let mut child = Command::new(cmdargs[0])
                .args(&cmdargs[1..])
                .spawn()
                .expect("failed to execute the command");

            let exit_code = child.wait().expect("wait failed for the child process");
            println!(
                "{} exited with {}\n",
                cmdargs[0].blue(),
                format!("{}", exit_code).blue()
            )
        }
        Ok(String::from(s))
    }
}
