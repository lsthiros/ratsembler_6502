# 6502 ELF Format
A major goal of this project is to take the compiled assembly and store it in
a relocatable ELF file. Traditionally speaking, there is no elf format for the
6502 instruction set, thus this format should be thought of as an ELF extension
for this very specific ABI.

## 6502 ELF writer design
This writer has been designed to work with a "relocatable" trait detailed in
relocatable.rs. The trait specifies the following behaviors:
 * `get_raw_section(&self) -> Vec<u8>`
 * `get_relocations(&self) -> Vec<Relcocation>`
 * `get_symbols(&self) -> Hashmap<String, Symbol>`

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
st_info: Info about what type of symbol this is. Part binding, part type.
| 7:4| 3:0|
|bind|type|

Bindings are defined by glibc as follows:
* `0x0` STB\_LOCAL Local symbol
* `0x1` STB\_GLOBAL Global symbol
* `0x2` STB\_WEAK Weak symbol
* `0x3` STB\_NUM Number of defined types.
* `0xA` STB\_LOOS Start of OS-specific
* `0xA` STB\_GNU\_UNIQUE Unique symbol. (These are the same?)
* `0xC` STB\_HIOS End of OS-specific
* `0xD` STB\_LOPROC Start of processor-specific
* `0xF` STB\_HIPROC End of processor-specific

Types are defined by glibc as follows:
* `0x0` STT\_NOTYPE Symbol type is unspecified
* `0x1` STT\_OBJECT Symbol is a data object
* `0x2` STT\_FUNC Symbol is a code object
* `0x3` STT\_SECT Symbol associated with a section
* `0x4` STT\_FILE Symbol's name is file name
* `0x5` STT\_COMMON Symbol is a common data object
* `0x6` STT\_TLS Symbol is thread-local data object
* `0x7` STT\_NUM Number of defined types.
* `0xA` STT\_LOOS Start of OS-specific
* `0xA` STT\_GNU_IFUNC Symbol is indirect code object
* `0xC` STT\_HIOS End of OS-specific
* `0xD` STT\_LOPROC Start of processor-specific
* `0xF` STT\_HIPROC End of processor-specific

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

### Relocation Table
The relocation table describes portions of sections that need to be modified
during the linking process. Relocation tables come in two flavors, `rel` and
`rela`. The entries of these tables look something like this:

```
Elf32_Rel: {
    r_offset: u32,
    r_info: u32
}
```
```
Elf32_Rela: {
    r_offset: u32,
    r_info: u32,
    r_addend: i32
}
```
`rela` entries include an explicit addend that may be useful for particular
architectures. For the 6502 architecture, there doesn't seem to be a good use
for the addend field, so we will just be using `rel` for now.

`r_offset` gives the location of the relocation as an index into a section. For
an executable file, this contains the virtual address of the relocation.

`r_info` is a compound field containing a symbol and type
|   31:8|    7:0|
| R\_SYM|R\_TYPE|

R\_SYM refers to the symbol listed in this relocation table's associated symbol
table. According to CMU, "If the index is STN_UNDEF \[...\] the relocation uses
0 as the 'symbol value'".

Relocation types are heavily tied to the ISA. For our assembler, we seem to use
four different types that I made without properly documenting them at the time
\(whoops\). I believe that they look something like this

|code|type    |discription                                                   |
+----+--------+--------------------------------------------------------------+
|0x00|NO_TYPE |Undefined. Neither used nor supported                         |
|0x01|SHORT   |Reserved                                                      |
|0x02|LONG    |Reserved                                                      |
|0x03|ABS     |An absolute address from a label                              |
|0x04|RELATIVE|An offset from the current PC to labeled address e.g. branches|

# Sources
[CMU Material on ELF Files](https://web.archive.org/web/20241224203513/https://www.cs.cmu.edu/afs/cs/academic/class/15213-f00/docs/elf.pdf)

[ayedaemon's Post on String Tables](https://ayedaemon.github.io/post/2023/10/elf-chronicles-string-tables/)

[ayedaemon's Post on Symbol Tables](https://ayedaemon.github.io/post/2023/10/elf-chronicles-symbol-tables/)

[ayedaemon's Post on Relocation Tables](https://ayedaemon.github.io/post/2023/12/elf-chronicles-relocations/)
