use std::{fs::File, io::Write, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = File::create(path).unwrap();
    write!(file, "{}", content).unwrap()
}
