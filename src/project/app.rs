use regex::Regex;
use std::process::Command;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
    pub package: String,
    pub manager: String,
}

//todo support more than one manager using an enum

impl App {
    pub fn check(&self) -> bool {
        let output = match self.manager.trim() {
            "apt" => {
                // not sure if this is the best way to find out if the package is there but it seems to work
                let o = Command::new("dpkg")
                    .args(&["-s", &self.name])
                    .output()
                    .expect("failed to execute dpkg when checking if a package is installed");
                let re = Regex::new(r"install ok installed").unwrap();
                re.is_match(&String::from_utf8_lossy(&o.stdout))
            }
            _ => false,
        };
        output
    }
}
