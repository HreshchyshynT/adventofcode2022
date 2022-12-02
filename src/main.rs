mod first;
mod second;
mod utils;
use std::io;

fn main() -> io::Result<()> {
    first::run()?;
    second::run()?;
    Ok(())
}
