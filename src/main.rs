mod first;
mod second;
mod third;
mod utils;
use std::io;

fn main() -> io::Result<()> {
    // first::run()?;
    // second::run()?;
    third::run()?;
    Ok(())
}
