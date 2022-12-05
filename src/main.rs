mod first;
mod second;
mod third;
mod utils;
use std::io;
mod fourth;

fn main() -> io::Result<()> {
    // first::run()?;
    // second::run()?;
    // third::run()?;
    fourth::run()?;
    Ok(())
}
