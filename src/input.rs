use std::{fs, io, path::PathBuf};

pub(crate) fn read_input_file(filename: &str) -> io::Result<String> {
    let mut filepath = PathBuf::from("input/");
    filepath.push(filename);

    fs::read_to_string(filepath)
}
