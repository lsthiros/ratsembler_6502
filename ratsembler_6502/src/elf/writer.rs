use std::fs::File;
use std::io::{self, Write};
use std::mem;

const EI_NIDENT: usize = 16;
const EM_6502: u16 = 0x6502; // Hypothetical value for 6502 architecture

#[repr(C)]
struct Elf32Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl Elf32Ehdr {
    fn new() -> Self {
        let mut e_ident = [0u8; EI_NIDENT];
        e_ident[0] = 0x7F;
        e_ident[1] = b'E';
        e_ident[2] = b'L';
        e_ident[3] = b'F';
        e_ident[4] = 1; // ELFCLASS32
        e_ident[5] = 1; // ELFDATA2LSB
        e_ident[6] = 1; // EV_CURRENT

        Elf32Ehdr {
            e_ident,
            e_type: 2, // ET_EXEC
            e_machine: EM_6502,
            e_version: 1, // EV_CURRENT
            e_entry: 0x08048000,
            e_phoff: mem::size_of::<Elf32Ehdr>() as u32,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: mem::size_of::<Elf32Ehdr>() as u16,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

pub fn write_elf_header(file: &mut File) -> io::Result<()> {
    let header = Elf32Ehdr::new();
    let header_bytes = unsafe {
        std::slice::from_raw_parts(
            &header as *const Elf32Ehdr as *const u8,
            mem::size_of::<Elf32Ehdr>(),
        )
    };
    file.write_all(header_bytes)?;
    Ok(())
}
