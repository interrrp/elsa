//! A CLI tool to list files in a directory.
//!
//! See the [GitHub repository](https://github.com/interrrp/elsa) for more information.

use clap::Parser;

use crate::lister::print_list_dir;

mod args;
mod lister;

fn main() {
    let args = args::Args::parse();
    print_list_dir(&args);
}
