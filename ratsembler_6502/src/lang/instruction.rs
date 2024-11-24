
pub enum InstructionCode {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    ADC,
    SBC,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    AND,
    ORA,
    EOR,
    JMP,
    BCC,
    BCS,
    BEQ,
    BNE,
    BMI,
    BPL,
    BVS,
    BVC,
    CMP,
    CPX,
    CPY,
    BIT,
    ASL,
    LSR,
    ROL,
    ROR,
    TSX,
    TXS,
    PHA,
    PHP,
    PLA,
    PLP,
    JSR,
    RTS,
    RTI,
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,
    NOP,
    BRK,
}
pub enum AddressMode {
    ACCUMULATOR,
    ABSOLUTE,
    ABS_X,
    ABS_Y,
    IMMEDIATE,
    REL_ZP,
    INDEX_IND,
    ZP_X,
    ZP_Y,
    IND_INDEX,
}