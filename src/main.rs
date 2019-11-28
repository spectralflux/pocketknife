extern crate clap;

use clap::{App, Arg};
use dialoguer::Confirmation;
use std::process::Command;
use std::str;

// scrub: runs `git reset --hard`, with confirmation before nuke.
fn scrub() {
    if Confirmation::new()
        .with_text("😎 Scrub a dub dub! Return git folder to a pristine state?")
        .default(false)
        .interact()
        .unwrap()
    {
        println!("");

        let output = Command::new("git")
            .arg("reset")
            .arg("--hard")
            .output()
            .expect("Failed to run command.");

        let sout = output.stdout;

        println!("{}", str::from_utf8(&sout).unwrap());
    } else {
        println!("Scrub cancelled.");
    }
}

fn main() {
    println!("☠ Pocketknife ☠");
    let matches = App::new("scrub")
        .args(&[Arg::with_name("command")
            .help("the command to use")
            .index(1)
            .required(false)])
        .get_matches();

    let command = matches.value_of("command");
    match command {
        None => println!(
            "Commands available:\n   scrub: runs `git reset --hard`, with confirmation before nuke."
        ),
        Some(command_str) => match command_str {
            "scrub" => scrub(),
            _ => println!("Unknown command."),
        },
    }
}
