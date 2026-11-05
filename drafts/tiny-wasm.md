# Tiny WebAssembly Compiler

Compile a small structured language into a WebAssembly module and run it in an existing runtime.


## First pass

Interpret a source language with integer arithmetic, locals, `if`, `while`, and an imported print function.
Use 32-bit integers to match WebAssembly semantics.


## Compiler extension

Lower the source into structured WebAssembly control flow.
Emit a binary `.wasm` module with one exported `main` function and a host-provided print import.
Use a stack-machine intermediate representation to make validation straightforward.


## Evidence of success

Run the generated module in a browser, Node.js, or a command-line WebAssembly runtime.
Inspect the emitted module with a disassembler and compare output to the interpreter.
Demonstrate a numeric workload such as Fibonacci or prime counting.


## Optional challenges

- Arithmetic simplification
- Loop hoisting
