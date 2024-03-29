use std::{path::PathBuf, sync::atomic::Ordering};

use crate::common::{assert_results, setup, tear_down};
use sar::core::{process_directory, TOTAL_FILES_EDITED_OK, TOTAL_FILES_TO_BE_EDITED};

mod common;

#[test]
fn it_searches_and_replaces_text_from_english_to_english_no_regex() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = vec!["json".to_string(), "js".to_string()];
            let ignored_dirs: Vec<String> = Vec::new();
            let search: String = "positive".to_string();
            let replace: String = "negative".to_string();
            let dry_run: bool = false;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 6);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 6);
            assert_results(
                "expected-21.json",
                "actual-2.json",
                tests_directory.as_path(),
            );
            assert_results("expected-31.js", "actual-3.js", tests_directory.as_path());
            assert_results("original-1.txt", "actual-1.txt", tests_directory.as_path());
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}

#[test]
fn it_searches_and_replaces_text_from_english_to_english_regex() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = vec!["txt".to_string()];
            let ignored_dirs: Vec<String> = Vec::new();
            let search: String = r"(?P<a>[a-z]{3})\.(?P<b>[a-z]{3})\.(?P<c>\d{3})".to_string();
            let replace: String = r"$a.$b.com".to_string();
            let dry_run: bool = false;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 3);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 3);
            assert_results("original-1.txt", "actual-1.txt", tests_directory.as_path());
            assert_results("expected-2.txt", "actual-2.txt", tests_directory.as_path());
            assert_results(
                "original-2.json",
                "actual-2.json",
                tests_directory.as_path(),
            );
            assert_results("original-3.js", "actual-3.js", tests_directory.as_path());
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}

#[test]
fn it_searches_and_replaces_text_from_spanish_to_malayalam() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = vec!["json".to_string(), "js".to_string()];
            let ignored_dirs: Vec<String> = Vec::new();
            let search: String = "número".to_string();
            let replace: String = "നമ്പർ".to_string();
            let dry_run: bool = false;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 6);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 6);
            assert_results(
                "expected-22.json",
                "actual-2.json",
                tests_directory.as_path(),
            );
            assert_results("expected-32.js", "actual-3.js", tests_directory.as_path());
            assert_results("original-1.txt", "actual-1.txt", tests_directory.as_path());
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}

#[test]
fn it_searches_but_wont_replace_text_in_dry_run_mode() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = vec!["json".to_string(), "js".to_string()];
            let ignored_dirs: Vec<String> = Vec::new();
            let search: String = "positive".to_string();
            let replace: String = "negative".to_string();
            let dry_run: bool = true;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 6);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 0);
            assert_results(
                "original-2.json",
                "actual-2.json",
                tests_directory.as_path(),
            );
            assert_results("original-3.js", "actual-3.js", tests_directory.as_path());
            assert_results("original-1.txt", "actual-1.txt", tests_directory.as_path());
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}

#[test]
fn it_wont_replace_text_when_search_text_is_not_found() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = vec!["json".to_string(), "js".to_string()];
            let ignored_dirs: Vec<String> = Vec::new();
            let search: String = "google".to_string();
            let replace: String = "".to_string();
            let dry_run: bool = false;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 0);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 0);
            assert_results(
                "original-2.json",
                "actual-2.json",
                tests_directory.as_path(),
            );
            assert_results("original-3.js", "actual-3.js", tests_directory.as_path());
            assert_results("original-1.txt", "actual-1.txt", tests_directory.as_path());
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}

#[test]
fn it_searches_and_replaces_text_from_all_files_when_file_extensions_are_omitted() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = Vec::new();
            let ignored_dirs: Vec<String> = Vec::new();
            let search: String = "positive".to_string();
            let replace: String = "negative".to_string();
            let dry_run: bool = false;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 9);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 9);
            assert_results(
                "expected-21.json",
                "actual-2.json",
                tests_directory.as_path(),
            );
            assert_results("expected-31.js", "actual-3.js", tests_directory.as_path());
            assert_results("expected-1.txt", "actual-1.txt", tests_directory.as_path());
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}
#[test]
fn it_searches_and_replaces_text_from_all_files_except_files_in_ignored_dir1_and_dir2() {
    match setup() {
        Ok(tests_directory) => {
            let test_run_directory: PathBuf =
                [tests_directory.display().to_string().as_str(), "test_run"]
                    .iter()
                    .collect();
            let file_extensions: Vec<String> = Vec::new();
            let ignored_dirs: Vec<String> = vec!["dir1".to_string(), "dir2".to_string()];
            let search: String = "positive".to_string();
            let replace: String = "negative".to_string();
            let dry_run: bool = false;
            process_directory(
                test_run_directory.as_path(),
                &file_extensions,
                &ignored_dirs,
                search.as_str(),
                replace.as_str(),
                &dry_run,
            );
            assert_eq!(TOTAL_FILES_TO_BE_EDITED.load(Ordering::Relaxed), 3);
            assert_eq!(TOTAL_FILES_EDITED_OK.load(Ordering::Relaxed), 3);
        }
        Err(error) => eprintln!("{}", error),
    }
    TOTAL_FILES_TO_BE_EDITED.store(0, Ordering::Relaxed);
    TOTAL_FILES_EDITED_OK.store(0, Ordering::Relaxed);
    match tear_down() {
        Ok(()) => println!("tear_down ok"),
        Err(error) => eprintln!("tear_down error: {}", error),
    }
}
