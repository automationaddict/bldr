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

    pub fn detect_linux_distribution() -> Result<String, String> {
        // get the package for this ditribution
        let file =
            File::open("/etc/os-release").map_err(|e| format!("Unable to open file {}", e))?;

        let reader = BufReader::new(file);

        // search for the "ID" filed in the file
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error reading line: {}", e))?;

            if let Some(rest) = line.strip_prefix("ID=") {
                return Ok(rest.trim_matches('"').to_string());
            }
        }
        Err("Distribution name not found".to_string())
    }
}

pub mod create_directories {

    use colored::Colorize;
    use core::panic;
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::path::Path;

    pub fn dotfiles() {
        // create/verify paths exist
        let home = env::var("HOME").expect("$HOME is not set");
        let mut paths = HashMap::new();

        #[rustfmt::skip]
    paths.insert(
        "config_dir", 
        Path::new("").join(&home).join(".config"));
        #[rustfmt::skip]
    paths.insert(
        "vault_dir",
        Path::new("").join(&home).join(".ansible-vault"));
        #[rustfmt::skip]
    paths.insert(
        "dotfiles_dir", 
        Path::new("").join(&home).join(".dotfiles"));
        #[rustfmt::skip]
    paths.insert(
        "ssh_dir", 
        Path::new("").join(&home).join(".ssh"));

        for (key, value) in paths {
            match value.try_exists() {
                Ok(ok) => {
                    if !ok {
                        match fs::create_dir(value) {
                            Ok(_) => println!(
                                "{} {} {}",
                                "Creating".bold().yellow(),
                                key.bold().yellow(),
                                "directory!".bold().yellow()
                            ),
                            Err(err) => panic!(
                                "{} {}",
                                "Cannot access directory: ".bold().red(),
                                err.to_string().bold().red()
                            ),
                        }
                    } else {
                        println!(
                            "{} {} {}",
                            key,
                            "exists! ->",
                            value.into_os_string().into_string().unwrap().green()
                        );
                    }
                }
                Err(_) => panic!(
                    "{}",
                    "Cannot confirm nor deny directory existence".bold().red()
                ),
            }
        }
    }
}
