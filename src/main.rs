extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Builder script")
        .version("0.1")
        .author("Rafael Antognolli <antognolli@gmail.com>")
        .about("Builds mesa, xserver, weston...")
        .arg(Arg::with_name("v")
             .short("v")
             .help("Verbose output - prints stdout and stderr"))
        .arg(Arg::with_name("PACKAGE")
             .help("List of packages to build")
             .index(1)
             .multiple(true))
        .get_matches();

    if matches.is_present("v") {
        println!("Printing verbose output...")
    }

    let pkglist = match matches.values_of("PACKAGE") {
        Some(values) => values.collect(),
        None => vec!["all"],
    };

    for pkg in pkglist {
        println!("printing pkg: {}", pkg);
    }
}
