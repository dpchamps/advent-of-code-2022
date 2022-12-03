use std::{env, fs};

pub fn get_data(day: &str) -> std::io::Result<String> {
    let path = format!(
        "{}/src/bin/{}/input.txt",
        env::current_dir()?.display(),
        day
    );

    fs::read_to_string(path)
}

pub fn get_lines(day: &str) -> std::io::Result<Vec<String>> {
    Ok(get_data(&day)?.lines().map(String::from).collect())
}
