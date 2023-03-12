use regex::Regex;
use std::borrow::Cow;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, LineWriter, Result, Write};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::{DirEntry, WalkDir};

pub static TOTAL_FILES_TO_BE_EDITED: AtomicUsize = AtomicUsize::new(0);
pub static TOTAL_FILES_EDITED_OK: AtomicUsize = AtomicUsize::new(0);
pub static TOTAL_FILES_PROCESS_ERROR: AtomicUsize = AtomicUsize::new(0);
pub static TOTAL_DIR_READ_ERROR: AtomicUsize = AtomicUsize::new(0);

pub fn process_directory(
    directory: &Path,
    file_extensions: &Vec<String>,
    ignored_dirs: &Vec<String>,
    search: &str,
    replace: &str,
    dry_run: &bool,
) {
    let regex: Regex = Regex::new(search).unwrap();
    let walker = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !is_directory_ignored(e, ignored_dirs));
    for entry in walker {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file()
                    && is_matching_file(entry.file_name().to_str(), file_extensions)
                {
                    match process_file(&entry.path(), &regex, replace, dry_run) {
                        Ok(()) => (),
                        Err(error) => {
                            eprintln!(
                                "Error processing file: {} -- {}",
                                entry.path().display(),
                                error
                            );
                            TOTAL_FILES_PROCESS_ERROR.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                TOTAL_DIR_READ_ERROR.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

fn is_directory_ignored(entry: &DirEntry, ignored_dirs: &Vec<String>) -> bool {
    entry.file_type().is_dir()
        && entry
            .file_name()
            .to_str()
            .map(|s| ignored_dirs.iter().any(|x| s.eq(x)))
            .unwrap_or(false)
}

fn is_matching_file(entry: Option<&str>, file_extensions: &Vec<String>) -> bool {
    entry
        .map(|s| {
            if file_extensions.len() == 0 {
                return true;
            } else {
                for file_extension in file_extensions {
                    if s.ends_with(file_extension) {
                        return true;
                    }
                }
                return false;
            }
        })
        .unwrap_or(false)
}

fn process_file(file_path: &Path, regex: &Regex, replace: &str, dry_run: &bool) -> Result<()> {
    let mut lines: Vec<String> = Vec::new();
    let mut found_and_replaced: bool = false;
    let file: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    loop {
        let mut line: String = String::new();
        let len: usize = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        } else {
            let result: Cow<'_, str> =
                search_and_replace(&line, regex, replace, &mut found_and_replaced);
            match result {
                Cow::Borrowed(_) => lines.push(line), // regex not matching, no replacement
                Cow::Owned(val) => lines.push(val.to_owned()), // regex matching, replaced
            }
        }
    }
    if found_and_replaced {
        println!("Search text found in file: {}", file_path.display());
        TOTAL_FILES_TO_BE_EDITED.fetch_add(1, Ordering::Relaxed);
        if !dry_run {
            remove_file(file_path)?;
            write_file(lines, file_path)?;
            println!("Successfully edited file: {}", file_path.display());
            TOTAL_FILES_EDITED_OK.fetch_add(1, Ordering::Relaxed);
        }
    }
    Ok(())
}

fn search_and_replace<'a>(
    line: &'a str,
    regex: &Regex,
    replace: &str,
    found_and_replaced: &mut bool,
) -> Cow<'a, str> {
    if regex.is_match(line) {
        *found_and_replaced = true;
        return regex.replace_all(line, replace);
    }
    return Cow::from(line);
}

fn write_file(lines: Vec<String>, path: &Path) -> Result<()> {
    let file: File = File::create(path)?;
    let mut writer: LineWriter<File> = LineWriter::new(file);
    for line in lines {
        writer.write_all(line.as_bytes())?;
    }
    writer.flush()?;
    Ok(())
}

pub fn validate_file_extensions(file_extensions: &Vec<String>) -> bool {
    for file_extension in file_extensions {
        if file_extension.starts_with(".") || file_extension.contains("*") {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::core::{is_matching_file, search_and_replace, validate_file_extensions};
    use regex::Regex;
    use std::borrow::Cow;
    #[test]
    fn search_and_replace_positive() {
        let mut found_and_replaced: bool = false;
        let original_text =
            "import { BrowserModule } @igniteui/ from \"@igniteui/platform-browser\";".to_string();
        let expected_text = "import { BrowserModule }  from \"platform-browser\";".to_string();
        let regex: Regex = Regex::new("@igniteui/").unwrap();
        let result: Cow<str> =
            search_and_replace(&original_text, &regex, "", &mut found_and_replaced);
        assert_eq!(result, expected_text);
        assert_eq!(found_and_replaced, true);
    }
    #[test]
    fn search_and_replace_negative() {
        let mut found_and_replaced: bool = false;
        let original_text = "import { BrowserModule } from \"platform-browser\";".to_string();
        let expected_text = "import { BrowserModule } from \"platform-browser\";".to_string();
        let regex: Regex = Regex::new("@igniteui/").unwrap();
        let result: Cow<str> =
            search_and_replace(&original_text, &regex, "", &mut found_and_replaced);
        assert_eq!(result, expected_text);
        assert_eq!(found_and_replaced, false);
    }

    #[test]
    fn search_and_replace_regex_positive_1() {
        let mut found_and_replaced: bool = false;
        let original_text = r"2012-03-14, 2013-01-01 and 2014-07-05".to_string();
        let expected_text = r"03/14/2012, 01/01/2013 and 07/05/2014".to_string();
        let regex: Regex = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        let result: Cow<str> =
            search_and_replace(&original_text, &regex, "$m/$d/$y", &mut found_and_replaced);
        assert_eq!(result, expected_text);
        assert_eq!(found_and_replaced, true);
    }

    #[test]
    fn search_and_replace_regex_positive_2() {
        let mut found_and_replaced: bool = false;
        let original_text = r"www.xyz.123".to_string();
        let expected_text = r"www.xyz.com".to_string();
        let regex: Regex = Regex::new(r"(?P<a>[a-z]{3})\.(?P<b>[a-z]{3})\.(?P<c>\d{3})").unwrap();
        let result: Cow<str> = search_and_replace(
            &original_text,
            &regex,
            r"$a.$b.com",
            &mut found_and_replaced,
        );
        assert_eq!(result, expected_text);
        assert_eq!(found_and_replaced, true);
    }

    #[test]
    fn search_and_replace_regex_negative() {
        let mut found_and_replaced: bool = false;
        let original_text = r"03/14/2012, 01/01/2013 and 07/05/2014".to_string();
        let expected_text = r"03/14/2012, 01/01/2013 and 07/05/2014".to_string();
        let regex: Regex = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        let result: Cow<str> =
            search_and_replace(&original_text, &regex, "$m/$d/$y", &mut found_and_replaced);
        assert_eq!(result, expected_text);
        assert_eq!(found_and_replaced, false);
    }

    #[test]
    fn is_matching_file_positive() {
        let file_extensions: Vec<String> = vec!["txt".to_string(), "json".to_string()];
        let entry: Option<&str> = Some("test.json");
        let is_matching_file = is_matching_file(entry, &file_extensions);
        assert_eq!(is_matching_file, true);
    }
    #[test]
    fn is_matching_file_negative() {
        let file_extensions: Vec<String> = vec!["txt".to_string(), "json".to_string()];
        let entry: Option<&str> = Some("test.js");
        let is_matching_file = is_matching_file(entry, &file_extensions);
        assert_eq!(is_matching_file, false);
    }
    #[test]
    fn is_matching_file_no_file_extensions() {
        let file_extensions: Vec<String> = Vec::new();
        let entry: Option<&str> = Some("test.js");
        let is_matching_file = is_matching_file(entry, &file_extensions);
        assert_eq!(is_matching_file, true);
    }
    #[test]
    fn validate_file_extensions_positive() {
        let file_extensions: Vec<String> = vec!["txt".to_string(), "json".to_string()];
        let validate_file_extensions = validate_file_extensions(&file_extensions);
        assert_eq!(validate_file_extensions, true);
    }
    #[test]
    fn validate_file_extensions_negative_1() {
        let file_extensions: Vec<String> = vec!["*.txt".to_string(), "json".to_string()];
        let validate_file_extensions = validate_file_extensions(&file_extensions);
        assert_eq!(validate_file_extensions, false);
    }
    #[test]
    fn validate_file_extensions_negative_2() {
        let file_extensions: Vec<String> = vec!["txt".to_string(), ".json".to_string()];
        let validate_file_extensions = validate_file_extensions(&file_extensions);
        assert_eq!(validate_file_extensions, false);
    }
}
