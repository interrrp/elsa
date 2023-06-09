use clap::Parser;

use crate::lister::print_list_dir;

mod args;
mod lister;

fn main() {
    let args = args::Args::parse();
    print_list_dir(&args);
}
