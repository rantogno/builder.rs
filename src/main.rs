extern crate clap;
use clap::{Arg, App};

fn pkg_print(pkg: &str) {
    println!("Building package: {}", pkg);
}

fn build_all() {
    println!("No packages specified. Building all of them.");
}

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

    let pkg_values = matches.values_of("PACKAGE");

    match pkg_values {
        None => build_all(),
        _ => pkg_values.unwrap().for_each(pkg_print),
    }

}
