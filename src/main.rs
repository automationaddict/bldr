use colored::Colorize;
use core::panic;
use sysinfo::System;
use which::which;
use whoami::Platform;

fn main() {
    // get users real name
    println!("{} {}", "Hello".green(), whoami::realname().green());

    let mut sys = System::new_all();

    sys.refresh_all();

    // get OS of the system
    match whoami::platform() {
        Platform::Linux => println!(
            "{} {} {} {}",
            "You are running Distro:".green(),
            whoami::distro().green(),
            "on Arch:".green(),
            whoami::arch().to_string().green()
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

    // check if python3 is installed
    match which("python3") {
        Ok(path) => println!(
            "{} {}",
            "Python available at:".green(),
            path.into_os_string().into_string().unwrap().green()
        ),
        Err(err) => println!(
            "{} {}",
            "Installing Python3".yellow(),
            err.to_string().yellow()
        ),
    };
}
