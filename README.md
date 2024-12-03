# Ratsembler 6502
A 6502 assembler using a basic grammar from an earlier project. This
application takes a file written in our 6502 Assembler language and
compiles it into a a Unix-style ELF file.

## Naming
As with my other odd projects named after creatures, this, too, is left
as an excercise for the reader.

## Motivation
This point of this project is largely to investigate the generalities
of the common ELF file structure, and the means by which linking is
undertaken. An acceptable final product is one that can produce object
files whose symbols and relocations can be investigated with traditional
readelf utilities. Shall I choose to go the extra mile, I'd like to put
together a linker to demonstrate to myself that I fully understand the
different pieces of the ELF format at play.

## Design
The application will take a singular `*.s` file as an argument. The
application will use the `pest` library to lex and parse the program
into an AST. The AST will be processed into a list of instructions. Some
instructions will refer to symbols in the ELF sense. These symbols will
need to be enumerated in the symbol table in the final elf output.

### The Parser
The parser will be designed to read a grammar describing a 6502 assembler
language reminiscent of the better-documented GNU Assembler. It shall
support labels to describe instruction positions, and may even support
a similar local label system. It may be missing many of the more useful
directives, if it even includes directives at all, which it may at some
point. It shall support all 6502 instructions in all addressing modes.

I will be using "pest" as the parser because thats the library I found
first. I prefer the EBNF-like syntax of flex + bison a good deal more,
but I can't see anything wrong with learning PEG.

#### A Parser Example

A very basic program might look like this:

```
JSR init
BRK

init:
LDX #$00
RTS
```

The AST might look something like this.

```
JSR
 |  \
init \
     BRK
       \
        \
         LDX
        / |  \
       / #$00 \
    init:     RTS
```

The parser should be able to transform this into a list of instructions and a
list of the labels with their positions like so:

Instructions:
1. JSR
  * Mode: Absolute
    * Label: "init"
2. BRK
  * Mode: Implied
3. LDX
  * Mode: Immediate
    * Value: 0x00
4. RTS
  * Mode Immediate

Labels:
1. init
  * Location: 0x04
