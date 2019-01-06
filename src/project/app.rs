use regex::Regex;
use std::process::Command;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
    pub package: String,
    pub manager: String, // this will move to it's own struct
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

    pub fn install(&self) -> bool {
        match self.manager.trim() {
            "apt" => {
                let o = Command::new("apt-get")
                    .args(&["install", "-y", &self.package])
                    .output()
                    .expect("package manager failed to install");
                println!("installing {}", &self.name);
                println!("{}", &String::from_utf8_lossy(&o.stdout));
                println!("{}", &String::from_utf8_lossy(&o.stderr));
                true
            }
            _ => {
                println!("Unable to install {}", &self.package);
                false
            }
        }
    }
}
