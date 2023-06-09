//! Contains directory listing logic (see `print_list_dir`).

/// Print the contents of a directory to stdout.
pub fn print_list_dir(dir_path: &str, show_hidden: bool) {
    let entries = std::fs::read_dir(dir_path).unwrap();
    for entry in entries {
        let file_name = entry.unwrap().file_name().into_string().unwrap();
        if file_name.starts_with(".") && !show_hidden {
            continue;
        }

        println!("{}", file_name);
    }
}
