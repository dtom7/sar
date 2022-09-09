use clap::{ArgAction, Parser};
use dialoguer::Confirm;
use sar::core::{
    process_directory, validate_file_extensions, TOTAL_DIR_READ_ERROR, TOTAL_FILES_EDITED_OK,
    TOTAL_FILES_PROCESS_ERROR, TOTAL_FILES_TO_BE_EDITED,
};
use std::{path::PathBuf, sync::atomic::Ordering};

#[derive(Parser, Default, Debug)]
#[clap(version, about)]
struct Arguments {
    #[clap(short)]
    /// Directory to search for files recursively. If omitted, current directory (".") will be taken. Usage: -d "C:\Temp"
    directory: Option<PathBuf>,
    #[clap(multiple = true, short = 'x')]
    /// File extension(s) to include in the search. If omitted, all file extensions will be included. Usage: -x txt (single file extension) or -x json -x txt (multiple file extensions) or -x json txt (multiple file extensions)
    file_extensions: Vec<String>,
    #[clap(forbid_empty_values = true, short)]
    /// Text to search in files, cannot be a blank ("") value. Usage: -s test
    search: String,
    #[clap(short)]
    /// Text to replace in files. If omitted, blank ("") value will be taken. Usage: -r test
    replace: Option<String>,
    #[clap(long = "dry", action = ArgAction::SetTrue)]
    /// Dry run option. No files will be modified. Just displays the files containing the search text. Usage: --dry
    dry_run: Option<bool>,
}

fn main() {
    let separator = "=".repeat(60);
    let args: Arguments = Arguments::parse();
    let directory: PathBuf = args.directory.unwrap_or_else(|| PathBuf::from("."));
    let file_extensions: Vec<String> = args.file_extensions;
    if !validate_file_extensions(&file_extensions) {
        println!("{}", separator);
        eprintln!("File extensions cannot contain '*' and cannot start with '.'");
        println!("{}", separator);
        std::process::exit(1);
    }
    let search: String = args.search;
    let replace: String = args.replace.unwrap_or_else(|| String::from(""));
    let dry_run: bool = args.dry_run.unwrap_or_else(|| false);
    let message: String = format!("In directory: \"{}\", for file extension(s): {:?}, search for: \"{}\" and replace with: \"{}\" and dry-run: {}",
    directory.display(), file_extensions, search, replace, dry_run);
    let count: usize = message.chars().count();
    println!("{}", "=".repeat(count));
    println!("{}", message);
    println!("{}", "=".repeat(count));
    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .default(true)
        .wait_for_newline(true)
        .interact()
        .expect("Error confirming user input")
    {
        process_directory(
            directory.as_path(),
            &file_extensions,
            search.as_str(),
            replace.as_str(),
            &dry_run,
        );
        println!("{}", separator);
        println!(
            "Total # of files where search text was found: {}",
            TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed)
        );
        println!(
            "Total # of files where search text was replaced: {}",
            TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed)
        );
        println!(
            "Total # of files not searched or edited (error): {}",
            TOTAL_FILES_PROCESS_ERROR.load(Ordering::Relaxed)
        );
        println!(
            "Total # of directories or files not entered (error): {}",
            TOTAL_DIR_READ_ERROR.load(Ordering::Relaxed)
        );
        println!("{}", separator);
    } else {
        std::process::exit(0);
    }
}
