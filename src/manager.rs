// Learning how patterns look in Rust...
struct Manager {
    name: String,
    version: String,
    command: String,
}

// Types of installers
enum PackageManager {
    Apt(Manager),
    Docker(Manager),
    Homebrew(Manager),
}

// Traits for the installers
trait PackageManagement {
    fn install_app(&self, app: String) {
        println!("Installing app {}!", app);
    }
}

trait ExternalPackageManager {
    fn install() {}
}

// Implementations
impl PackageManager {
    fn new_apt_manager() -> PackageManager {
        PackageManager::Apt(Manager {
            name: String::from("apt"),
            version: String::from("1.6.3"),
            command: String::from("apt"),
        })
    }
    fn new_homebrew_manager() -> PackageManager {
        PackageManager::Homebrew(Manager {
            name: String::from("homebrew"),
            version: String::from("latest"),
            command: String::from("brew"),
        })
    }
}

impl PackageManagement for PackageManager {}
