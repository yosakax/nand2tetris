// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, the
// program clears the screen, i.e. writes "white" in every pixel.

// R0: if keyboard is pushed in previous term then 1
// R1: if keyboard is pushed in current term then 1

@8192
D=A
@pixel
M=D
@i
M=0
@toBlack
M=1

(LOOP)
    //@toBlack
    //D=M
    @i
    M=0

    @KBD
    D=M
    // if key input not changed, back to loop
    @toBlack
    D=D-M
    @LOOP
    D;JEQ

    
    @KBD
    D=M
    @toBlack
    M=D

    @FILL_SCREEN_BLACK 
    D;JNE // if toBlack != 0 jmp

    @FILL_SCREEN_WHITE
    0;JMP


(FILL_SCREEN_BLACK)
    //@toBlack
    //M=0

    @i
    D=M // D = i
    @pixel
    D=M-D // D=pixel - i
    @LOOP
    D;JLT // if pixel - i <= 0 then jmp

    @SCREEN
    A=D+A
    M=-1
    @i
    MD=M+1
    @FILL_SCREEN_BLACK
    0;JMP


(FILL_SCREEN_WHITE)
    //@toBlack
    //M=1

    @i
    D=M
    @pixel
    D=M-D
    @LOOP
    D;JLT

    @SCREEN
    A=D+A
    M=0
    @i
    MD=M+1
    @FILL_SCREEN_WHITE
    0;JMP
