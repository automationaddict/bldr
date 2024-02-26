pub mod gather_info {

    use colored::Colorize;
    use core::panic;
    use std::process::Command;

    // custom error type for the get_users_full_name() function
    #[derive(Debug)]
    pub enum NameError {
        CommandExecutionError,
        UserInfoParsingError,
    }

    // get the full users name
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

    pub fn get_support() {
        //check if this tool is supported
        if sysinfo::IS_SUPPORTED_SYSTEM {
            println!("{}", "This OS is suported by this tool".green())
        } else {
            panic!("{}", "This OS is not supported by this tool".bold().red());
        }
    }
}

pub mod validate_sys {

    use colored::Colorize;
    use core::panic;

    pub fn distribution_check() {
        // get the package for this ditribution
        let dist = match whoami::devicename_os().into_string() {
            Ok(res) => res.to_lowercase().replace(' ', "_"),
            Err(err) => panic!(
                "{} {:?}",
                "Cannot get distribution name".bold().red(),
                err.into_string().unwrap().bold().red()
            ),
        };

        //  limit this to a single distribution for the moment
        if dist != "pop_os" {
            panic!(
                "{}",
                "Tooling is not configured for this distribution yet"
                    .bold()
                    .red()
            )
        }
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
                            key.green(),
                            "exists! ->".green(),
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
