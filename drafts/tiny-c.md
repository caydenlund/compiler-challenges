# Tiny C Subset

Compile a deliberately tiny C-like language with integer expressions, variables, functions, and control flow.


## First pass

Parse integer literals, local variables, `if`, `while`, function calls, and `return`.
Interpret an abstract syntax tree with 64-bit signed integers.
Provide a built-in `print` function for visible output.


## Compiler extension

Lower the syntax tree to a three-address intermediate representation with basic blocks.
Generate native assembly that follows the platform calling convention for a restricted function signature.


## Evidence of success

Compile and run programs that calculate Fibonacci numbers, prime counts, or a text-mode Mandelbrot.
Show the intermediate representation and emitted assembly for one program.
Compare interpreted and compiled run times on a loop-heavy workload.


## Optional challenges

- Constant folding
- Dead-block removal
- Undefined behavior sanitizer
- Loop hoisting
- Floating-point values (including casting)
