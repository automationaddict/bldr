use bldr::create_directories;
use bldr::gather_info;
use bldr::gather_info::NameError;
use bldr::validate_sys;
use colored::Colorize;
use std::process::Command;
use std::str;
use which::which;

fn main() {
    // greet the user with their full name
    match gather_info::get_users_full_name() {
        Ok(full_name) => println!(
            "{} {}",
            "Full Name:".truecolor(255, 120, 0),
            full_name.truecolor(255, 120, 0)
        ),
        Err(err) => match err {
            NameError::CommandExecutionError => {
                println!("{}", "Error executing system command".bold().red())
            }
            NameError::UserInfoParsingError => {
                println!("{}", "Unable to retrieve user information".bold().red())
            }
        },
    }

    // get the OS that this is installed on
    gather_info::get_os();

    // check if this tool is supported
    gather_info::get_support();

    // check if this distribution is supported
    validate_sys::distribution_check();

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
    update_cache();

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
