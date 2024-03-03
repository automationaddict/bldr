use std::process::Command;

fn apt_update_upgrade() {
    // update and upgrade the system
    let _ = Command::new("sudo")
        .arg("apt")
        .arg("update")
        .output()
        .expect("Failed to update the system");

    let _ = Command::new("sudo")
        .arg("apt")
        .arg("upgrade")
        .arg("-y")
        .output()
        .expect("Failed to upgrade the system");
}

// custom error type for the install_packages() function
#[derive(Debug)]
pub enum InstallError {
    CommandExecutionError,
}

pub fn install_package(package: &str) -> Result<(), InstallError> {
    // update the apt cache
    apt_update_upgrade();

    // HACK: even though this is ugly, it is the best way to check if the repo is setup without requiring the user to add the ppa
    if package.contains("ansible") {
        // check if the software-properties-common package is installed
        let output = Command::new("dpkg")
            .arg("-l")
            .arg("software-properties-common")
            .output()
            .map_err(|_| InstallError::CommandExecutionError)?;
        // if the package is not installed
        // install it and add the ansible repository
        if !output.status.success() {
            // install the package
            Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("-y")
                .arg("software-properties-common")
                .output()
                .map_err(|_| InstallError::CommandExecutionError)?;
        }
        // check if the ansible repository is added
        // if not add it
        let repo = Command::new("sh")
            .arg("-c")
            .arg("grep -q '^deb .*ansible/ansible' /etc/apt/sources.list /etc/apt/sources.list.d/*")
            .status()
            .map_err(|_| InstallError::CommandExecutionError)?;

        if !repo.success() {
            Command::new("sudo")
                .arg("apt-add-repository")
                .arg("--yes")
                .arg("--update")
                .arg("ppa:ansible/ansible")
                .output()
                .map_err(|_| InstallError::CommandExecutionError)?;
        }
        if !output.status.success() || !repo.success() {
            return Err(InstallError::CommandExecutionError);
        }
    }
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
