//! Contains command-line argument definitions using [Clap](https://clap.rs).

use clap::Parser;

/// The command-line arguments.
#[derive(Parser)]
pub struct Args {
    /// The path to directory to list. If not specified, the current directory is used.
    #[arg(default_value = ".")]
    pub dir_path: String,
}
