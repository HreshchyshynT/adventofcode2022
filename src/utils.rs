use std::fs::File;
use std::io::BufReader;

pub fn read_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
