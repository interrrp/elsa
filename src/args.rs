//! Contains command-line argument definitions using [Clap](https://clap.rs).

use std::path::PathBuf;

use clap::Parser;

/// The command-line arguments.
#[derive(Parser)]
pub struct Args {
    /// The path to directory to list. If not specified, the current directory is used.
    #[arg(default_value = ".")]
    pub dir_path: PathBuf,

    /// Whether to show hidden files (files starting with a dot).
    #[arg(short, long)]
    pub all: bool,

    /// Whether to append newlines to the end of each filename.
    #[arg(short, long)]
    pub newline: bool,

    /// Whether to append an emoji correlating to the file type.
    #[arg(short, long)]
    pub emoji: bool,
}
