# Befunge Trace JIT Compiler

Implement a small Befunge-93 interpreter, then JIT-compile frequently executed instruction traces.


## First pass

Interpret the instructions in the 80x25 playfield, including directional execution, the integer stack, arithmetic, conditionals, string mode, and character I/O.
Make each executed instruction available in a textual trace.


## Compiler extension

Profile program-counter transitions while interpreting.
Form traces for common paths through the 2D control-flow graph.
Compile arithmetic-heavy traces to native code or a simple bytecode.
Guard every compiled trace at branches and fall back to the interpreter when its assumptions fail.

Target Befunge-93 only.
Treat self-modifying playfield writes as trace-invalidating events rather than solving arbitrary code mutation.


## Evidence of success

Show an execution heat map of the playfield.
Run a small animation or factorial program before and after trace compilation.
Report trace counts, guard exits, and elapsed time.
