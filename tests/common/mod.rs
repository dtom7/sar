use std::{
    env,
    fs::{copy, create_dir_all, read_to_string, remove_dir_all},
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
    slice::Iter,
};

pub fn setup() -> Result<PathBuf> {
    tear_down()?;
    let mut current_dir: PathBuf = env::current_dir()?;
    println!("setup current directory: {}", current_dir.display());
    current_dir.push("tests");
    if Path::new(current_dir.as_os_str()).try_exists()? {
        create_directories(
            [
                current_dir.display().to_string().as_str(),
                "test_run",
                "dir1",
                "dir11",
            ]
            .iter(),
        )?;
        create_directories(
            [
                current_dir.display().to_string().as_str(),
                "test_run",
                "dir2",
            ]
            .iter(),
        )?;
        copy_files("original-1.txt", "actual-1.txt")?;
        copy_files("original-2.json", "actual-2.json")?;
        copy_files("original-3.js", "actual-3.js")?;
        Ok(current_dir)
    } else {
        let mut message = "tests directory not found in current directory: ".to_string();
        message.push_str(current_dir.display().to_string().as_str());
        Err(Error::new(ErrorKind::NotFound, message.as_str()))
    }
}

pub fn tear_down() -> Result<()> {
    let mut current_dir: PathBuf = env::current_dir()?;
    println!("tear_down current directory: {}", current_dir.display());
    current_dir.push("tests");
    current_dir.push("test_run"); // ./tests/test_run
    if Path::new(current_dir.as_os_str()).try_exists()? {
        println!("{} exists", current_dir.display());
        remove_dir_all(current_dir.as_path())?;
    } else {
        println!("{} doesn't exist", current_dir.display());
    }
    Ok(())
}

pub fn assert_results(expected_file_name: &str, actual_file_name: &str, tests_directory: &Path) {
    let source_file_path: PathBuf = [
        tests_directory.display().to_string().as_str(),
        "resources",
        expected_file_name,
    ]
    .iter()
    .collect();
    let target_file_path: PathBuf = [
        tests_directory.display().to_string().as_str(),
        "test_run",
        actual_file_name,
    ]
    .iter()
    .collect();
    compare_file_contents(source_file_path.as_path(), target_file_path.as_path());
    let target_file_path: PathBuf = [
        tests_directory.display().to_string().as_str(),
        "test_run",
        "dir2",
        actual_file_name,
    ]
    .iter()
    .collect();
    compare_file_contents(source_file_path.as_path(), target_file_path.as_path());
    let target_file_path: PathBuf = [
        tests_directory.display().to_string().as_str(),
        "test_run",
        "dir1",
        "dir11",
        actual_file_name,
    ]
    .iter()
    .collect();
    compare_file_contents(source_file_path.as_path(), target_file_path.as_path());
}

fn compare_file_contents(source_file_path: &Path, target_file_path: &Path) {
    let source_file: Result<String> = read_to_string(source_file_path);
    let target_file: Result<String> = read_to_string(target_file_path);
    println!("source_file: {}", source_file_path.display());
    let source_file_contents = source_file.expect("unable to read source file");
    let target_file_contents = target_file.expect("unable to read source file");
    assert_eq!(source_file_contents, target_file_contents);
}

fn create_directories(paths: Iter<&str>) -> Result<()> {
    let target_directory: PathBuf = paths.collect();
    println!("{}", target_directory.display());
    create_dir_all(target_directory.as_path())?;
    Ok(())
}

fn copy_files(source_file_name: &str, target_file_name: &str) -> Result<()> {
    let current_dir: PathBuf = env::current_dir()?;
    let text_file_from: PathBuf = [
        current_dir.display().to_string().as_str(),
        "tests",
        "resources",
        source_file_name,
    ]
    .iter()
    .collect();
    let text_file_to: PathBuf = [
        current_dir.display().to_string().as_str(),
        "tests",
        "test_run",
        "dir1",
        "dir11",
        target_file_name,
    ]
    .iter()
    .collect();
    copy(&text_file_from, &text_file_to)?;
    let text_file_to: PathBuf = [
        current_dir.display().to_string().as_str(),
        "tests",
        "test_run",
        "dir2",
        target_file_name,
    ]
    .iter()
    .collect();
    copy(&text_file_from, &text_file_to)?;
    let text_file_to: PathBuf = [
        current_dir.display().to_string().as_str(),
        "tests",
        "test_run",
        target_file_name,
    ]
    .iter()
    .collect();
    copy(text_file_from, text_file_to)?;
    Ok(())
}
