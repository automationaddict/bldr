pub mod gather_info {

    use std::process::Command;

    // custom error type for the get_users_full_name()
    #[derive(Debug)]
    pub enum NameError {
        CommandExecutionError,
        UserInfoParsingError,
    }

    // get the users full name
    pub fn get_users_full_name() -> std::result::Result<String, NameError> {
        // get username using unix whoami
        let output = Command::new("whoami")
            .output()
            .map_err(|_| NameError::CommandExecutionError)?;

        let username = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // use specific information from the getent unix name service library
        let user_info_output = Command::new("getent")
            .arg("passwd")
            .arg(&username)
            .output()
            .map_err(|_| NameError::CommandExecutionError)?;

        let user_info = String::from_utf8_lossy(&user_info_output.stdout);
        let fields: Vec<&str> = user_info.split(':').collect();

        if fields.len() >= 5 {
            // the GECOS file (index 4) contains the full name
            Ok(fields[4].trim().to_string())
        } else {
            Err(NameError::UserInfoParsingError)
        }
    }

    // custom error type for the get_os_info() function
    #[derive(Debug)]
    pub enum OSError {
        CommandExecutionError,
        OsInfoParsingError,
    }

    // get the os name and version
    pub fn get_os_info() -> Result<String, OSError> {
        // get the os information from '/etc/os-release'
        let output = Command::new("cat")
            .arg("/etc/os-release")
            .output()
            .map_err(|_| OSError::CommandExecutionError)?;

        // could get PRETTY_NAME but would need to parse it later
        let os_info = String::from_utf8_lossy(&output.stdout);
        let mut os_name = String::new();
        let mut os_version = String::new();

        for line in os_info.lines() {
            if let Some(rest) = line.strip_prefix("NAME=") {
                os_name = rest.trim_matches('"').to_string();
            } else if let Some(rest) = line.strip_prefix("VERSION=") {
                os_version = rest.trim_matches('"').to_string();
            }
        }

        if !os_name.is_empty() && !os_version.is_empty() {
            Ok(format!("{} {}", os_name, os_version))
        } else {
            Err(OSError::OsInfoParsingError)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_users_full_name() {
            assert!(get_users_full_name().is_ok());
        }

        #[test]
        fn test_get_os_info() {
            assert!(get_os_info().is_ok());
        }
    }
}

pub mod validate_sys {

    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // custom error type for the get_platorm() function
    // not necessary but allows expansion later
    #[derive(Debug)]
    pub enum PlatformError {
        PlatformInfoParsingError,
    }

    // check which platform the tool is running on
    pub fn get_platform() -> Result<String, PlatformError> {
        //check if this tool is supported
        let os_type = env::consts::OS;
        match os_type {
            // todo: add additional OS later
            "linux" => Ok("Linux".to_string()),
            _ => Err(PlatformError::PlatformInfoParsingError),
        }
    }

    // custom error type for the detect_linux_distribution() function
    #[derive(Debug)]
    pub enum DistroError {
        CommandExecutionError,
        DistributionInfoParsingError,
        DistributionNameNotFound,
    }

    pub fn get_linux_distribution() -> Result<String, DistroError> {
        // get the package for this ditribution
        let file = File::open("/etc/os-release").map_err(|_| DistroError::CommandExecutionError)?;

        let reader = BufReader::new(file);

        // search for the "ID" filed in the file
        for line in reader.lines() {
            let line = line.map_err(|_| DistroError::DistributionInfoParsingError)?;

            if let Some(rest) = line.strip_prefix("ID=") {
                return Ok(rest.trim_matches('"').to_string());
            }
        }
        Err(DistroError::DistributionNameNotFound)
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_platform() {
            assert!(get_platform().is_ok());
        }

        #[test]
        fn test_get_linux_distribution() {
            assert!(get_linux_distribution().is_ok());
        }
    }
}

pub mod dotfiles_presence {

    use std::fs;
    use std::path::PathBuf;

    // custom error type for the setup_dotfiles_directories() function
    #[derive(Debug)]
    pub enum DotfilesError {
        DirectoryCreationError,
        DirectoryAccessError,
        DirectoryPresenceError,
    }

    pub fn dotfiles_directories(path: &PathBuf) -> Result<bool, DotfilesError> {
        match path.try_exists() {
            Ok(ok) => {
                if ok {
                    // directory exists so return true
                    Ok(true)
                } else {
                    // directory does not exist so create it
                    // fs::create_dir(path).map_err(|_| DotfilesError::DirectoryCreationError)?;
                    let res = fs::create_dir(path);
                    match res {
                        Ok(_) => Ok(false),
                        Err(_) => Err(DotfilesError::DirectoryCreationError),
                    }
                }
            }
            Err(_) => {
                // cannot access the directory
                Err(DotfilesError::DirectoryAccessError)
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env;
        use std::path::Path;

        #[test]
        fn test_dotfiles_directories() {
            let home = env::var("HOME").unwrap();
            let mut path = Path::new(&home).join(".config");
            assert!(dotfiles_directories(&path).is_ok());

            path = Path::new(&home).join(".ansible-vault");
            assert!(dotfiles_directories(&path).is_ok());

            path = Path::new(&home).join(".dotfiles");
            assert!(dotfiles_directories(&path).is_ok());

            path = Path::new(&home).join(".ssh");
            assert!(dotfiles_directories(&path).is_ok());
        }
    }
}

pub mod install_packages {

    use std::process::Command;

    // custom error type for the install_packages() function
    #[derive(Debug)]
    pub enum InstallError {
        CommandExecutionError,
    }

    pub fn install_package(package: &str) -> Result<(), InstallError> {
        // install the packages
        let output = Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg(package)
            .output()
            .map_err(|_| InstallError::CommandExecutionError)?;

        if output.status.success() {
            Ok(())
        } else {
            Err(InstallError::CommandExecutionError)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_install_package() {
            assert!(install_package("curl").is_ok());
        }
    }
}
