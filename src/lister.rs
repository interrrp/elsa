//! Contains directory listing logic (see `print_list_dir`).

use owo_colors::OwoColorize;

use crate::args::Args;

/// Print the contents of a directory to stdout.
pub fn print_list_dir(args: &Args) {
    let entries = std::fs::read_dir(&args.dir_path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();

        let file_name = entry.file_name().into_string().unwrap();
        let hidden = file_name.starts_with(".");
        if hidden && !args.all {
            continue;
        }
        let file_type = entry.file_type().unwrap();

        if !args.no_emoji {
            if file_type.is_dir() {
                print!("📁 ");
            } else {
                let extension = extension(&file_name).unwrap_or("");
                print!("{} ", emoji_for(&extension));
            }
        }

        if hidden {
            print!("{}", file_name.dimmed());
        } else {
            print!("{}", file_name);
        }

        if args.newline {
            println!();
        } else {
            print!("  ");
        }
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

/// Return the extension of a file name.
fn extension(file_name: &str) -> Option<&str> {
    let mut parts = file_name.split('.');
    let _ = parts.next();
    parts.next()
}
