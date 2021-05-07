use std::fs;
use std::io::Result;

fn main() {
    let path = "/".to_string(); // need get from command param
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
