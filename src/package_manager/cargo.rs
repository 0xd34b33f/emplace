use super::{CaptureFlag, PackageInstalledMethod, PackageManagerTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Cargo;

impl PackageManagerTrait for Cargo {
    fn full_name(self) -> &'static str {
        "Cargo Rust"
    }

    fn commands(self) -> Vec<&'static str> {
        vec!["cargo"]
    }

    fn sub_commands(self) -> Vec<&'static str> {
        vec!["install"]
    }

    fn install_command(self) -> &'static str {
        "cargo install --quiet"
    }

    fn needs_root(self) -> bool {
        false
    }

    #[cfg(not(target_os = "windows"))]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!(
            "cargo install --list | grep 'v[0-9]' | grep -q {}",
            package
        ))
    }
    #[cfg(target_os = "windows")]
    fn is_installed(self, package: &str) -> PackageInstalledMethod {
        PackageInstalledMethod::Script(format!("cargo install --list | findstr {}", package))
    }

    fn known_flags_with_values(self) -> Vec<&'static str> {
        vec!["-Z", "--version", "-j", "--jobs"]
    }

    fn capture_flags(self) -> Vec<CaptureFlag> {
        vec![CaptureFlag::Single("--git")]
    }
}

#[cfg(test)]
mod tests {
    use super::Cargo;
    use crate::{catch, package_manager::PackageManager};

    #[test]
    fn test_package_manager() {
        let manager = PackageManager::from_line("cargo install test").unwrap();
        assert_eq!(manager, PackageManager::from(Cargo));
    }

    #[test]
    fn test_catch() {
        // Regular invocation
        catch!(PackageManager::from(Cargo), "cargo install test" => "test");

        // Multiple
        catch!(PackageManager::from(Cargo), "cargo install test test2" => "test", "test2");

        // Flags that should be captured
        catch!(PackageManager::from(Cargo), "cargo install --git https://test.com/test.git" => "https://test.com/test.git" ["--git"]);
    }
}
