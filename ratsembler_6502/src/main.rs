mod elf;

use std::fs::File;
use std::io::{self};

use elf::writer::write_elf_header;
fn main() -> io::Result<()> {
    let mut file = File::create("output_6502.elf")?;
    write_elf_header(&mut file)?;
    Ok(())
}
