use clap::Parser;

mod args;

fn main() {
    let args = args::Args::parse();

    if &args.dir_path == "." {
        println!("Listing the current directory");
    } else {
        println!("Listing directory {}", args.dir_path);
    }
}
