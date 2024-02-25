pub mod gather_info {

    use colored::Colorize;
    use core::panic;
    use sysinfo::System;
    use whoami::Platform;

    pub fn get_user() {
        // get users real name
        println!("{} {}", "Hello".magenta(), whoami::realname().magenta());
    }

    pub fn get_os() {
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
    }

    pub fn get_support() {
        //check if this tool is supported
        if sysinfo::IS_SUPPORTED_SYSTEM {
            println!("{}", "This OS is suported by this tool".green())
        } else {
            panic!("{}", "This OS is not supported by this tool".red());
        }
    }
}

pub mod validate_sys {

    use colored::Colorize;
    use core::panic;

    pub fn distribution_check() {
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
    }
}
