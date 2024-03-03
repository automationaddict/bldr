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
