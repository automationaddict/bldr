use bldr::dotfiles_presence;
use bldr::dotfiles_presence::DotfilesError;
use bldr::gather_info;
use bldr::gather_info::NameError;
use bldr::gather_info::OSError;
use bldr::validate_sys;
use bldr::validate_sys::PlatformError;
use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;
use which::which;

fn main() -> ExitCode {
    // greet the user with their full name
    match gather_info::get_users_full_name() {
        Ok(full_name) => println!("{} {}", "Hey!", full_name.green()),
        Err(err) => match err {
            NameError::CommandExecutionError => {
                eprintln!("{}", "Error executing system command".bold().red());
                return ExitCode::FAILURE;
            }
            NameError::UserInfoParsingError => {
                eprintln!("{}", "Unable to retrieve user information".bold().red());
                return ExitCode::FAILURE;
            }
        },
    }

    // get the OS that this is installed on
    match gather_info::get_os_info() {
        Ok(os_info) => println!("{} {}", "Operating System:", os_info.green()),
        Err(err) => match err {
            OSError::CommandExecutionError => {
                println!("{}", "Error executing system command".bold().red());
                return ExitCode::FAILURE;
            }
            OSError::OsInfoParsingError => {
                println!("{}", "Unable to retrieve OS information".bold().red());
                return ExitCode::FAILURE;
            }
        },
    }

    // check if this tool is supported
    match validate_sys::get_platform() {
        Ok(platform) => println!("{} {}", "Your Platform is supported:", platform.green()),
        Err(err) => match err {
            PlatformError::PlatformInfoParsingError => {
                eprintln!("{}", "Your Platform is NOT supported".bold().red());
                return ExitCode::FAILURE;
            }
        },
    }

    // check if this distribution is supported
    match validate_sys::get_linux_distribution() {
        Ok(distro) => println!("{} {}", "Detected distribution:", distro.green()),
        Err(err) => match err {
            validate_sys::DistroError::CommandExecutionError => {
                eprintln!("{}", "Error executing system command".bold().red());
                return ExitCode::FAILURE;
            }
            validate_sys::DistroError::DistributionInfoParsingError => {
                eprintln!(
                    "{}",
                    "Unable to retrieve distribution information".bold().red()
                );
                return ExitCode::FAILURE;
            }
            validate_sys::DistroError::DistributionNameNotFound => {
                eprintln!("{}", "Distribution name not found".bold().red());
                return ExitCode::FAILURE;
            }
        },
    }
    // get the home directory
    let home = env::var("HOME")
        .map_err(|_| {
            eprintln!("{}", "Error retrieving environment variables".bold().red());
            ExitCode::FAILURE
        })
        .unwrap();

    // create the paths
    let mut paths = HashMap::new();

    // todo: would these be better as a struct?
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

    for (_, path) in paths.iter() {
        // create/check existence of: dotfile directories
        match dotfiles_presence::dotfiles_directories(&path) {
            Ok(present) => {
                if present {
                    println!(
                        "{} {} {}",
                        "Dotfile directory",
                        path.to_owned()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                            .green(),
                        "is present"
                    );
                    return ExitCode::SUCCESS;
                }
                println!(
                    "{} {} {}",
                    "Dotfile directory".yellow(),
                    path.to_owned()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .yellow(),
                    "created successfully".yellow()
                );
                return ExitCode::SUCCESS;
            }
            Err(err) => match err {
                DotfilesError::DirectoryCreationError => {
                    eprintln!("{}", "Error creating dotfiles directories".bold().red());
                    return ExitCode::FAILURE;
                }
                DotfilesError::DirectoryAccessError => {
                    eprintln!("{}", "Error accessing dotfiles directories".bold().red());
                    return ExitCode::FAILURE;
                }
                DotfilesError::DirectoryPresenceError => {
                    eprintln!("{}", "Dotfiles directories are not present".bold().red());
                    return ExitCode::FAILURE;
                }
            },
        }
    }
    // packages to install/check
    let packages = vec![
        String::from("ssh"),
        String::from("git"),
        String::from("python3"),
        String::from("pip3"),
        String::from("ansible"),
    ];

    // update the apt cache
    // update_cache();

    // update the system before installing packages
    // upgrade_packages();

    for package in packages {
        match which(&package) {
            Ok(path) => println!(
                "{} {} {}",
                package,
                "available at:",
                path.into_os_string()
                    .into_string()
                    .expect("Unhandled path error")
                    .green()
            ),
            Err(err) => {
                eprintln!(
                    "{} {} {} {}",
                    "Installing".bold().yellow(),
                    package.bold().yellow(),
                    "->".bold().yellow(),
                    err.to_string().bold().yellow()
                );

                match &package[..] {
                    "ssh" => install_ssh(),
                    "git" => install_git(),
                    "python3" => install_python3(),
                    "pip3" => install_pip3(),
                    "ansible" => install_ansible(),
                    _ => eprintln!("No package to install"),
                };
            }
        };
    }
    ExitCode::SUCCESS
}

#[allow(dead_code)]
fn update_cache() {
    let _ = Command::new("sh")
        .arg("-c")
        .arg("sudo apt update")
        .output()
        .expect("failed to execute process");

    // let s = str::from_utf8(&output.stdout).unwrap();
    // println!("{}", s);
}

#[allow(dead_code)]
fn upgrade_packages() {
    todo!("upgrading packages")
}

#[allow(dead_code)]
fn install_ssh() {
    todo!("installing ssh")
}

#[allow(dead_code)]
fn install_git() {
    todo!("installing git")
}

#[allow(dead_code)]
fn install_python3() {
    todo!("installing python3")
}

#[allow(dead_code)]
fn install_pip3() {
    todo!("installing python3-pip")
}

#[allow(dead_code)]
fn install_ansible() {
    todo!("installing ansible")
}
