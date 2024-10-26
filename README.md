# Ratsembler 6502
A 6502 assembler using a basic grammar from an earlier project. This
application takes a file written in our 6502 Assembler language and
compiles it into a a Unix-style ELF file.

## Design
The application will take a singular `*.s` file as an argument. The
application will use the `pest` library to lex and parse the program
into an AST. The AST will be processed into a list of instructions. Some
instructions will refer to symbols in the ELF sense. These symbols will
need to be enumerated in the symbol table in the final elf output.
