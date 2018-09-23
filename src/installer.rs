use parser::{App, Project, OS};
use std::error::Error;

pub struct Installer {
    pub os: OS,
}

impl Installer {
    pub fn intsall_all(&self, applications: Vec<App>) -> Result<String, Box<Error>> {
        for app in applications {
            println!("Installing {}", app.name)
        }
        Ok(String::from("Everything is working now."))
    }

    pub fn install(&self, application: App) -> Result<String, Box<Error>> {
        Ok(String::from("Installed the application"))
    }
}
