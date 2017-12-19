extern crate clap;
use clap::{Arg, App, SubCommand};

struct Config {
    verbose: bool,
    cmd: Command,
}

struct InstallOptions {
    force: bool,
}

enum Command {
    Install{options: InstallOptions},
    Clean,
    Invalid,
}

static PACKAGES: &'static [&str] = &[
    "libdrm",
    "mesa"
];

// Check pkg list for invalid items
//  - returns list if valid, and None if some pkg names are not valid.
fn parse_pkg_list(pkglist: Vec<&str>) -> Option<Vec<&str>> {
    let mut notfound = false;

    for pkg in &pkglist {
        if !PACKAGES.contains(&pkg) {
            println!("Invalid pkgname: {}", pkg);
            notfound = true;
        }
    }

    match notfound {
        true => None,
        false => Some(pkglist),
    }
}

// Get pkg list from ArgMatches.
//  - returns Some(values) if pkglist is valid or the default list if
//    no list is provided.
//  - returns None if the list is invalid
fn get_pkg_list<'a>(matches: &'a clap::ArgMatches) -> Option<Vec<&'a str>> {
    let pkglist = match matches.values_of("PACKAGE") {
        Some(values) => parse_pkg_list(values.collect()),
        None => Some(PACKAGES.to_vec()),
    };

    pkglist
}

fn parse_install(matches: &clap::ArgMatches) -> InstallOptions {
    match get_pkg_list(matches) {
        Some(pkglist) => {
            for p in pkglist {
                println!("pkg: {}", p);
            }
        }
        None => println!("Invalid package in list... aborting.")
    }
    InstallOptions{force: false}
}

fn parse_matches(matches: &clap::ArgMatches) -> Config {
    let v = if matches.is_present("v") {
        true
    } else {
        false
    };
    Config{
        verbose: v,
        cmd: match matches.subcommand() {
            ("install", Some(sub_m)) => {
                Command::Install{options: parse_install(&sub_m)}
            }
            ("clean", Some(_)) => Command::Clean,
            (&_, Some(_)) => Command::Invalid,
            (&_, None) => Command::Invalid,
        },
    }
}

fn main() {
    let matches = App::new("Builder script")
        .version("0.1")
        .author("Rafael Antognolli <antognolli@gmail.com>")
        .about("Builds mesa, xserver, weston...")
        .arg(Arg::with_name("v")
             .short("v")
             .help("Verbose output - prints stdout and stderr"))
        .subcommand(SubCommand::with_name("install")
                    .about("Build and install package")
                    .arg(Arg::with_name("PACKAGE")
                         .help("List of packages to build")
                         .index(1)
                         .multiple(true)))
        .get_matches();

    let config = parse_matches(&matches);
    println!("{:?}", config.verbose);
}
