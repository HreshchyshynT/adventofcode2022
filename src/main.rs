mod day_5;
mod first;
mod fourth;
mod second;
mod third;
mod utils;

use std::io;

fn main() -> io::Result<()> {
    // first::run()?;
    // second::run()?;
    // third::run()?;
    // fourth::run()?;
    day_5::run()?;
    Ok(())
}
