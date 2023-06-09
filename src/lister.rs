//! Contains directory listing logic (see `print_list_dir`).

use owo_colors::OwoColorize;

use crate::args::Args;

/// Print the contents of a directory to stdout.
pub fn print_list_dir(args: &Args) {
    // Tilde expansion (e.g. `~` -> `/home/username` or `C:\Users\username` on Windows).
    let dir_path = shellexpand::tilde(&args.dir_path.to_string_lossy()).to_string();
    let path_buf = std::path::PathBuf::from(dir_path.as_ref() as &std::ffi::OsStr);

    let entries = std::fs::read_dir(path_buf).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        print_entry(&entry, &args);
    }
}

/// Print a single directory entry to stdout.
fn print_entry(entry: &std::fs::DirEntry, args: &Args) {
    let file_name = entry.file_name();
    let hidden = file_name.to_string_lossy().starts_with(".");
    if hidden && !args.all {
        return;
    }

    let file_type = entry.file_type().unwrap();

    if !args.no_emoji {
        if file_type.is_dir() {
            print!("📁 ");
        } else {
            let extension = entry
                .path()
                .extension()
                .unwrap_or(std::ffi::OsStr::new(".txt"))
                .to_owned();
            print!("{} ", emoji_for(extension.to_str().unwrap()));
        }
    }

    if hidden {
        print!("{}", file_name.to_string_lossy().dimmed());
    } else {
        print!("{}", file_name.to_string_lossy());
    }

    if args.newline {
        println!();
    } else {
        print!("  ");
    }
}

/// Return an emoji corresponding to the file extension.
///
/// This will try its best to return the most appropriate emoji for the file extension, otherwise it
/// will return a generic file emoji (📄).
fn emoji_for(extension: &str) -> &'static str {
    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "ico" | "bmp" | "tiff" | "tif" => "🖼️",
        "mp4" | "webm" | "mkv" | "mov" | "avi" | "wmv" | "mpg" | "mpeg" | "m4v" | "flv" => "🎞️",
        "mp3" | "wav" | "ogg" | "flac" | "m4a" | "wma" | "aac" | "opus" => "🎵",
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst" | "lzma" | "cab" | "iso"
        | "dmg" | "pkg" | "deb" | "rpm" | "apk" => "🗜️",
        "exe" | "msi" | "bat" | "sh" | "cmd" | "app" | "elf" | "so" | "dll" => "🖥️",
        _ => "📄",
    }
}
