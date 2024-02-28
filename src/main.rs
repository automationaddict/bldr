use bldr::create_directories;
use bldr::gather_info;
use bldr::gather_info::NameError;
use bldr::gather_info::OSError;
use bldr::validate_sys;
use colored::Colorize;
use std::process::Command;
use std::process::ExitCode;
use std::str;
use which::which;

fn main() -> ExitCode {
    // greet the user with their full name
    match gather_info::get_users_full_name() {
        Ok(full_name) => println!(
            "{} {}",
            "Hey!".truecolor(255, 120, 0),
            full_name.truecolor(255, 120, 0)
        ),
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
        Ok(os_info) => println!(
            "{} {}",
            "Operating System:".truecolor(255, 120, 0),
            os_info.truecolor(255, 120, 0)
        ),
        Err(err) => match err {
            OSError::CommandExecutionError => {
                println!("{}", "Error executing system command".bold().red())
            }
            OSError::OsInfoParsingError => {
                println!("{}", "Unable to retrieve OS information".bold().red())
            }
        },
    }

    // check if this tool is supported
    match validate_sys::get_platform() {
        Ok(os) => println!("{} {}", "Your OS is supported:".green(), os.green()),
        Err(err) => eprintln!(
            "{} {}",
            "Your OS is NOT supported yet".bold().red(),
            err.bold().red()
        ),
    }

    // check if this distribution is supported
    match validate_sys::detect_linux_distribution() {
        Ok(distro) => println!("{} {}", "Detected distribution:".green(), distro.green()),
        Err(err) => eprintln!(
            "{} {}",
            "Error detecting distribution:".bold().red(),
            err.bold().red()
        ),
    }

    // create/check existence of: dotfile directories
    create_directories::dotfiles();

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
    upgrade_packages();

    for package in packages {
        match which(&package) {
            Ok(path) => println!(
                "{} {} {}",
                package.green(),
                "available at:".green(),
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

fn update_cache() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo apt update")
        .output()
        .expect("failed to execute process");

    let s = str::from_utf8(&output.stdout).unwrap();
    println!("{}", s);
}

fn upgrade_packages() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo apt upgrade -y")
        .output()
        .expect("failed to execute process");

    let s = str::from_utf8(&output.stdout).unwrap();
    println!("{}", s);
}

fn install_ssh() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo apt install -y ssh")
        .output()
        .expect("failed to execute process");

    let s = str::from_utf8(&output.stdout).unwrap();
    println!("{}", s);
}

fn install_git() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo apt install -y git")
        .output()
        .expect("failed to execute process");

    let s = str::from_utf8(&output.stdout).unwrap();
    println!("{}", s);
}

fn install_python3() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("sudo apt install -y python3")
        .output()
        .expect("failed to execute process");

    let s = str::from_utf8(&output.stdout).unwrap();
    println!("{}", s);
}

#[allow(dead_code)]
fn install_pip3() {
    todo!("installing python3-pip")
}

#[allow(dead_code)]
fn install_ansible() {
    todo!("installing ansible")
}
