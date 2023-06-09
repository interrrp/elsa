use clap::Parser;

mod args;

fn main() {
    let args = args::Args::parse();

    let entries = std::fs::read_dir(&args.dir_path).unwrap();
    for entry in entries {
        let file_name = entry.unwrap().file_name();

        println!("{}", file_name.to_string_lossy());
    }
}
