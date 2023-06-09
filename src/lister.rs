//! Contains directory listing logic (see `print_list_dir`).

use crate::args::Args;

/// Print the contents of a directory to stdout.
pub fn print_list_dir(args: &Args) {
    let entries = std::fs::read_dir(&args.dir_path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();

        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with(".") && !args.all {
            continue;
        }

        println!("{}", file_name);
    }
}
