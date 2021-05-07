use std::fs;
use std::io::Result;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = match args.get(1) {
        Some(first) => first.to_string(),
        None => String::from(".")
    };

    path_iterator(path);
}

fn path_iterator(dir: String) -> Result<()> {
    let root = fs::read_dir(dir).unwrap();

    for dir in root {
        let path = dir.unwrap().path();
        let name = path.display();

        let realsize = match get_filesize(&path) {
            Ok(size) => size,
            Err(_) => break // need tratative
        };

        println!("{:?} bytes -> {:>15}", realsize, name)
    }
    Ok(())
}

fn get_filesize(path: &std::path::PathBuf) -> Result<u64> {
    let x = fs::metadata(&path)?.len();
    Ok(x)
}
