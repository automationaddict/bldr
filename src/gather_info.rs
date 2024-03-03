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

    // NOTE: could get PRETTY_NAME but would need to parse it later
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
