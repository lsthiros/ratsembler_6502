# 6502 ELF Format
A major goal of this project is to take the compiled assembly and store it in
a relocatable ELF file. Traditionally speaking, there is no elf format for the
6502 instruction set, thus this format should be thought of as an ELF extension
for this very specific ABI.

## 6502 ELF writer design
This writer has been designed to work with a "relocatable" trait detailed in
relocatable.rs. The trait specifies the following behaviors:
 * get_raw_section(&self) -> Vec<u8>
 * get_relocations(&self) -> Vec<Relcocation>
 * get_symbols(&self) -> Hashmap<String, Symbol>

Symbol and Relocation are sum types described as follows:

```
Symbol := {
    Location(usize),
    ShortValue(u8),
    LongValue(u16),
}

Relocation := {
    Absolute(String, u16),
    Relative(String, u16),
    Short(String, u16),
    Long(String, u16),
}
```

These behaviors should be all we need to construct a relocatable ELF file. The
file will have to have the following:
1. A header that denotes that this is a 6502 relocatable elf file
2. No program headers
3. Section data: The raw blocks as follows
 1. raw progbits from get_raw_sections, just the raw bits
 2. Symbol table
 3. Relocation table
 4. String table of symbols from the symbol table
 5. Section headers string table of the following strings
  1. ".text"
  2. ".symtab"
  3. ".rel.text"
  4. ".strtab"
  5. ".shstrtab"
3. Section headers immediately following the ELF header denoting the following
 1. The NULL section header
 2. ".text", PROGBITS, The actual program, from get_raw_section
 3. ".symtab", SYMTAB, The symbols from get_symbols
 4. ".rel.text", REL, The relocations from get_relocations
 5. ".strtab", SHT_STRTAB, the symbols from get_symbols
 6. ".shstrtab", SHT_STRTAB, the section table names

### String table section
The string table section is a contiguous array of null terminated strings. The
first string is always an empty string, e.g. a singular `\0`. 

### Symbol table section
The symbol table is a contiguous array of symbol entries. As with the other
tables, the 0th index serves as an undefined index. A symbol is defined
as

```
symbol: {
    st_name: u32,
    st_value: u32,
    st_size: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16
}
```

st_name: A nonzero offset into the string table that stores the symbol name
st_value: The value of the symbol. Could be an absolute value, address, etc.
st_size: Size of the symbol. Could be zero if size is undefined.
st_info: Info about what type of symbol this is. Part binding, part type. See
below

Bindings are defined by glibc as follows:
* `0x0` STB_LOCAL Local symbol
* `0x1` STB_GLOBAL Global symbol
* `0x2` STB_WEAK Weak symbol
* `0x3` STB_NUM Number of defined types.
* `0xA` STB_LOOS Start of OS-specific
* `0xA` STB_GNU_UNIQUE Unique symbol. (These are the same?)
* `0xC` STB_HIOS End of OS-specific
* `0xD` STB_LOPROC Start of processor-specific
* `0xF` STB_HIPROC End of processor-specific

Types are defined by glibc as follows:
* `0x0` STT_NOTYPE Symbol type is unspecified
* `0x1` STT_OBJECT Symbol is a data object
* `0x2` STT_FUNC Symbol is a code object
* `0x3` STT_SECT Symbol associated with a section
* `0x4` STT_FILE Symbol's name is file name
* `0x5` STT_COMMON Symbol is a common data object
* `0x6` STT_TLS Symbol is thread-local data object
* `0x7` STT_NUM Number of defined types.
* `0xA` STT_LOOS Start of OS-specific
* `0xA` STT_GNU_IFUNC Symbol is indirect code object
* `0xC` STT_HIOS End of OS-specific
* `0xD` STT_LOPROC Start of processor-specific
* `0xF` STT_HIPROC End of processor-specific

They are encoded to a `u8` using the following:
```
info(bind: u8, type: u8) -> u8 {(bind << 4) | type}
```

st_other: Just 0x00. Doesn't mean anything
st_shndx: Symbols are defined relative to some other section. e.g. a symbol
containing the address of a function in .text will point to .text's section
header here.

Some header indices are reserved. The first, SHN_UNDEF or `0x0000`, refers to
symbols that are not defined in the ELF file. The linker may fill in additional 
details if the symbol is defined elsewhere. Other reserved indices start at 
SHN_LORESERVE, or `0xFF00`. SHN_ABS, or `0xFFF1` refers to symbols that do not
change with relocation.

Another special index, SHN_COMMON or `0xFFF2` refers to a "common symbol" like
an unallocated C external variable. This special index requires that `st_value`
contain the alignment requirement for the symbol.

