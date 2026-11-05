# ARM to x86 or x86 to ARM Binary Translator

Translate a tiny AArch64 function subset into an equivalent x86-64 function, or vice versa.
IMHO, ARM is easier to decode, so ARM -> x86 is my recommendation.


## First pass

Decode and interpret a chosen subset of machine instructions.
Include at least the following:
- Integer arithmetic and comparisons
- Conditional branches
- Loads and stores; specifically, on the stack frame and in pointers passed as function arguments
- Return


## Compiler extension

Translate whole functions from the supported ARM/x86 subset to x86/ARM machine code or assembly.
Map source registers to a fixed virtual-register representation before allocating host registers.
Implement condition flags only for the instructions and branches you support.


## Evidence of success

Translate simple integer computations, including array summation/median/max/min, or a tiny checksum.
Run both the interpreter and your translated function against randomized test inputs and check for equality.
Show source and target disassembly with matching results.


## Optional challenges

- Use a library to decode an object file or executable instead of inputting raw instruction bytes
- Function calls and system calls
- Floating-point math or vector registers


# TODO

Is there an elegant way to inform the application about the given function signature?
Is that even needed for simple programs?
Maybe `x0` could just map unconditionally to `rdi`, etc.
