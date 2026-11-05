# Object File Repackager

Repackage a (tightly controlled) ELF executable into a Mach-O executable (or vice versa) without changing its machine code.


## First pass

Write a format inspector that parses ELF or Mach-O headers, load commands or program headers, sections, and symbols.
Print a normalized report.


## Compiler extension

Accept an AArch64 or x86-64 position-independent ELF or Mach-O program that uses no dynamic loader, relocations, or system calls beyond a supplied shim.
Lay out its text and data in a minimal executable of the opposite object file format with the equivalent entry point.
Emit a small Mach-O loader stub if the target as needed.
Reject unsupported object file extensions with a clear diagnostic message.


## Evidence of success

Convert a no-library hello-world or arithmetic demo and run it on the destination platform.
Show matching disassembly snippets and a file-format report before and after conversion.
