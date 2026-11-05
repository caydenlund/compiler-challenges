# Brainfuck to Native Code

Build a Brainfuck implementation that runs the classic esoteric language and is able to turn instructions into native machine code.
Whether you want to write an AOT compiler that generates an executable, a JIT compiler that immediately compiles and executes the whole program, or a JIT program that only compiles hot loops to machine code is up to you.


## First pass

Interpret the eight Brainfuck commands over a fixed byte tape.
This will be useful for checking that the behavior of your compiler is correct.


## Compiler extension

Parse matching brackets once and lower the program into a compact instruction stream.
Compile straight-line instruction runs and loops to native machine code.
Recognize simple zeroing loops such as `[-]` and `[+]`, and optionally multiplication-transfer loops like `[->>+++<<]` (i.e., `cell[pos + 2] += 3 * cell[pos]; cell[pos] = 0;`).

Use a documented fixed tape size.


## Evidence of success

There are lots of Brainfuck programs out there; make sure your interpreter and compiler are capable of running each one and produce the same output.
Render Mandelbrot ASCII art quickly.
Compare interpreter and compiled elapsed time on a loop-heavy program.
Include generated assembly or a disassembly beside the benchmark.
