// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

//// Replace this comment with your code.
    @R2
    M=0 // R2 = 0
    @i
    M=1 // i = 1
(LOOP)
    @i 
    D=M // D = i
    @R0
    D=D-M // i - R0
    @END
    D;JGT // if i - R0 > 0 JUMP
    @R1
    D=M // D = R1
    @R2
    M=D+M // R2 = R2 + R1
    @i
    M=M+1 // i++;    
    @LOOP
    0;JMP

(END)
    @END
    0;JMP


