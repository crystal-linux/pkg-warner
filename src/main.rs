#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use clap::Parser;
use std::{env, ops::Deref, process::Command};

mod args;
mod internal;

use args::Args;
use internal::{warn, AppExitCode};

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Get the current executable name as a String
    let bin = env::current_exe()
        .unwrap()
        .as_path()
        .file_name()
        .unwrap()
        .deref()
        .to_string_lossy()
        .to_string();

    // Get variables from the environment at build time to simplify the code
    let pkgs = env!("PKG_WARNER_PACKAGES")
        .split(',')
        .collect::<Vec<&str>>();
    let pman = env!("PKG_WARNER_PMAN").to_string();
    let dist = env!("PKG_WARNER_DISTRO").to_string();

    // If --test is specified, print the warn message and exit
    if args.test.unwrap_or(false) {
        warn(&bin, &dist, &pman);
    }

    // Check if the binary is called directly by the user, if so tell them off
    if bin == "pkg-warner" {
        if args.init.unwrap_or(false) {
            init(&pkgs, args.dest_dir);
        } else {
            crash!(
                AppExitCode::CalledDirectly,
                "`pkg-warner` is not meant to be called directly by the user"
            );
        }
    } else {
        warn(&bin, &dist, &pman);
    }
}

fn init(pkgs: &[&str], dest_dir: Option<String>) {
    log!("Initializing: {}", pkgs.join(", "));

    // Either unwrap the dest_dir if present or use /usr/bin
    let dest_dir = dest_dir.unwrap_or_else(|| "/usr/bin".to_string());

    for pkg in pkgs {
        log!("Installing \"{}\"", pkg);

        Command::new("install")
            .arg("-Dm0755")
            .arg(format!("{}", env::current_exe().unwrap().display()))
            .arg(format!("{}/{}", dest_dir, pkg))
            .spawn()
            .unwrap();
    }
}
