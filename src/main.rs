use colored::Colorize;
use core::panic;
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
            Err(err) => println!(
                "{} {} {} {}",
                "Installing".bold().yellow(),
                package.bold().yellow(),
                "->".bold().yellow(),
                err.to_string().bold().yellow()
            ),
        };
    }
}
