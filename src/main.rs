pub mod vsoc;

use std::{fs::File, io::Read};

use crate::vsoc::Vsoc;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    binary: String,
}

fn main() {
    let vsoc_name: String = String::from("RV64IM");
    let mut vsoc: Vsoc = Vsoc::new(&vsoc_name);
    let args = Args::parse();
    let mut file = File::open(args.binary).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    println!("> vemu");
    println!("{}", vsoc);
    println!("> vemu: vsoc: load flash...");
    println!("> vemu: vsoc: {}: run...", vsoc_name);
    vsoc.load(&contents);
    loop {
        if let Some(e) = vsoc.step() {
            println!("Exception: {}", e);

            println!("< vemu: vsoc: {}: halt", vsoc_name);
            println!("{}", vsoc);
            println!("< vemu");
            return;
        }
    }
}
