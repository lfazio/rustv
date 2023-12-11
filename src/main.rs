pub mod vsoc;

use std::{fs::File, io::Read};

use crate::vsoc::Vsoc;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Binary path
    #[arg(short, long)]
    binary: String,

    /// Vsoc description
    #[arg(short, long)]
    arch: String,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(&args.binary).unwrap();
    let mut contents = Vec::new();
    let vsoc_name: String = args.arch;
    let mut vsoc: Vsoc = Vsoc::new(&vsoc_name);

    println!("> vemu");
    println!("{}", vsoc);
    println!("> vemu: vsoc: load flash from {}...", &args.binary);
    file.read_to_end(&mut contents).unwrap();
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
