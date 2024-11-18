  JSR init
  JSR loop
  JSR end

init:
  LDX #$00
  RTS

loop:
  INX
  CPX #$05
  BNE loop
  RTS

end:
  BRK
  
LDX #$08
decrement:
DEX
STX $0200
CPX #$03
BNE decrement
STX $0201
BRK
lda #$11
sta $10
lda #$10
sta $12
lda #$0f
sta $14
lda #$04
sta $11
sta $13
sta $15
