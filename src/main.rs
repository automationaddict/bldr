use colored::Colorize;
use core::panic;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use sysinfo::System;
use which::which;
use whoami::Platform;

fn main() {
    // get users real name
    println!("{} {}", "Hello".magenta(), whoami::realname().magenta());

    let mut sys = System::new_all();

    sys.refresh_all();

    // get OS of the system
    match whoami::platform() {
        Platform::Linux => println!(
            "{} {} {} {}",
            "You are running Distro:".magenta(),
            whoami::distro().magenta(),
            "on Arch:".magenta(),
            whoami::arch().to_string().magenta()
        ),
        Platform::Unknown(s) => eprintln!("Unknown OS {}", s.red()),
        _ => panic!("{}", "Cannot identify the OS".red()),
    }

    //check if this tool is supported
    if sysinfo::IS_SUPPORTED_SYSTEM {
        println!("{}", "This OS is suported by this tool".green())
    } else {
        panic!("{}", "This OS is not supported by this tool".red());
    }

    // get the package for this ditribution
    let dist = match whoami::devicename_os().into_string() {
        Ok(res) => res.to_lowercase().replace(' ', "_"),
        Err(err) => panic!(
            "{} {:?}",
            "Cannot get distribution name".red(),
            err.into_string().unwrap().red()
        ),
    };

    //  limit this to a single distribution for the moment
    if dist != "pop_os" {
        panic!(
            "{}",
            "Tooling is not configured for this distribution yet".red()
        )
    }

    // create/verify paths exist
    let home = env::var("HOME").expect("$HOME is not set");
    let mut paths = HashMap::new();

    #[rustfmt::skip]
    paths.insert(
        "config_dir", 
        PathBuf::new().join(&home).join(".config"));
    #[rustfmt::skip]
    paths.insert(
        "vault_dir",
        PathBuf::new().join(&home).join(".ansible-vault"));
    #[rustfmt::skip]
    paths.insert(
        "dotfiles_dir", 
        PathBuf::new().join(&home).join(".dotfiles"));
    #[rustfmt::skip]
    paths.insert(
        "ssh_dir", 
        PathBuf::new().join(&home).join(".ssh"));

    println!(
        "PATH Value{:?}",
        paths.get(&"dotfiles_dir").unwrap().try_exists()
    );

    // packages to install/check
    let packages = vec![
        String::from("brew"),
        String::from("ssh"),
        String::from("git"),
        String::from("python3"),
        String::from("pip3"),
        String::from("ansible"),
    ];

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
                    "brew" => install_brew(),
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

#[allow(dead_code)]
fn install_brew() {
    todo!("installing brew")
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
