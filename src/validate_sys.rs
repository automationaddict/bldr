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
        // TODO: add additional OS later
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
