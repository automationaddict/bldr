use bldr::create_directories;
use bldr::gather_info;
use bldr::validate_sys;
use colored::Colorize;
use which::which;

fn main() {
    // greet the user with first and last name
    gather_info::get_user();

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
