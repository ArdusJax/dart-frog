extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod app;
mod env;
mod os;
mod plan;

use colored::*;
use project::app::App;
use project::os::OS;
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

// functions
pub fn parse_plan(path: &String) -> Result<String, Box<Error>> {
    // todo: This should use an actual file path object, not a string
    let mut cf = File::open(path).expect(&format!("{} file was not found", path));

    let mut contents = String::new();
    cf.read_to_string(&mut contents)
        .expect("There was and issue reading the continuum.yaml file.");
    Ok(contents)
}

impl Project {
    //todo implement resolving the requirements
    pub fn resolve_requirements(&self) -> Result<String, Box<Error>> {
        println!(
            "\u{1F3E0}{:>24}\n\n{:<15}\n",
            "resolving requirements".purple(),
            "installing:".green(),
        );
        for app in &self.apps {
            if !app.check() {
                app.install();
            }
            println!(
                "{:<2}{:<30}{:<15}",
                "\u{25AB}",
                app.name.yellow(),
                "installed".blue()
            );
        }
        Ok(String::from("all requirements have been gathered"))
    }
    // Execute the plan after the requirements have been gathered
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
