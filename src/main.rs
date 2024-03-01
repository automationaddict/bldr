use bldr::dotfiles_presence;
use bldr::dotfiles_presence::DotfilesError;
use bldr::gather_info;
use bldr::gather_info::NameError;
use bldr::gather_info::OSError;
use bldr::install_packages;
use bldr::validate_sys;
use bldr::validate_sys::PlatformError;
use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::path::Path;
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
                        "Dotfile directory ->",
                        path.to_owned()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                            .green(),
                        "is present"
                    );
                } else {
                    println!(
                        "{} {} {}",
                        "Dotfile directory ->",
                        path.to_owned()
                            .into_os_string()
                            .into_string()
                            .unwrap()
                            .bold()
                            .yellow(),
                        "created successfully"
                    );
                }
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

    // package struct
    struct Package {
        name: String,
        package: String,
    }

    // packages to check for
    let ssh = Package {
        name: String::from("ssh"),
        package: String::from("openssh-client"),
    };
    let git = Package {
        name: String::from("git"),
        package: String::from("git"),
    };
    let python3 = Package {
        name: String::from("python3"),
        package: String::from("python3"),
    };
    let python3_pip = Package {
        name: String::from("pip"),
        package: String::from("python3-pip"),
    };
    let ansible = Package {
        name: String::from("ansible"),
        package: String::from("ansible"),
    };

    let packages = vec![ssh, git, python3, python3_pip, ansible];

    println!(
        "{}",
        "Checking for required packages, please be patient!"
            .bold()
            .yellow()
    );

    // todo: need to handle this better. Maybe map on the
    // packages and return a result? Callbacks?
    //
    // loop through the packages and check if they are installed
    // if not, install them
    for package in packages.iter() {
        match which(&package.name) {
            Ok(path) => {
                println!(
                    "{} {} {}",
                    package.name,
                    "available at ->",
                    path.into_os_string()
                        .into_string()
                        .expect("Unhandled path error")
                        .green()
                );
            }
            Err(err) => match install_packages::install_package(&package.package) {
                Ok(_) => {
                    eprintln!(
                        "{} {} {} {}",
                        "Installing",
                        package.name.bold().yellow(),
                        "->",
                        err.to_string().bold().yellow()
                    );
                }
                Err(err) => match err {
                    install_packages::InstallError::CommandExecutionError => {
                        eprintln!("{}", "Error executing system command".bold().red());
                        return ExitCode::FAILURE;
                    }
                },
            },
        }
    }
    ExitCode::SUCCESS
}
