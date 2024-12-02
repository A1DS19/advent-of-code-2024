use std::{
    fs,
    io::{self, Read},
};

pub fn load_input(input: &str) -> io::Result<String> {
    let mut file = fs::File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
