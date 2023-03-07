use clap::Parser;

mod args;

fn main() {
    let args = dbg!(args::Args::parse());
}
