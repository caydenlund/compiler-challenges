# Spreadsheet Formula Compiler

Build a miniature spreadsheet engine that builds a dependency graph and incremental evaluation plan for updated values.


## First pass

Support named cells, floating-point values, arithmetic formulas, and a few functions such as `SUM` and `MIN`.
Interpret each formula directly whenever a cell is read.


## Compiler extension

Parse formulas into an intermediate representation and build a dependency graph.
Optionally, compile formulas into bytecode or native closures.
Recompute only cells reachable from an edited input and detect cycles with a useful error message.


## Evidence of success

Render a table of values before and after edits.
Show the dependency graph and the exact cells recalculated for each edit.
Benchmark repeated edits in a sheet with shared subexpressions.


## TODO

I can't decide how to specify the way that updates are made.

1. In source, eager evaluation
  Evaluation is done eagerly, as soon as a cell gets updated; `dumptable!()` only shows the table state, and does no evaluation on its own.
  Adding a `dumptable!()` command inside the source is super ugly, though.

  ```
  # eagerly set a1
  a1 = 1
  # eagerly set a2
  a2 = 2
  # eagerly set a3
  a3 = 3

  # eagerly set b1
  b1 = SUM(a1:a3)
  # eagerly set b2
  b2 = MIN(a1:a3)
  # eagerly set b3
  b3 = b1 + b2

  # eagerly update a1 -> [b1, b2] -> b3
  a1 = 2
  dumptable!()

  # eagerly update a2 -> [b1, b2] -> b3
  a2 = 4
  # eagerly update a3 -> [b1, b2] -> b3
  a3 = 6
  dumptable!()
  ```

  Then, all source cells must be defined before the cell users.
  This might make it annoying to write because then you can't simply alphabetize cells, but it forbids cycles unless a cell is updated.

2. In source, explicit evaluation
  Evaluation is done lazily, when the explicit `dumptable!()` command is given.
  Adding a `dumptable!()` command inside the source is (still) super ugly.

  ```
  a1 = 1
  a2 = 2
  a3 = 3

  b1 = SUM(a1:a3)
  b2 = MIN(a1:a3)
  b3 = b1 + b2

  # evaluate [a1, a2, a3] -> [b1, b2] -> b3
  dumptable!()

  a1 = 2
  # evaluate a1 -> [b1, b2] -> b3
  dumptable!()

  a2 = 4
  a3 = 6
  # evaluate [a2, a3] -> [b1, b2] -> b3
  dumptable!()
  ```

  I really like that in this version, the evaluator must explicitly track which values need to be updated and in which order, all at once.

3. Evaluation after each batch file
  Input is given as a series of files, and after each file is read, evaluation is performed.
  This avoids `dumptable!()`, but then each benchmark actually has a whole set of files that must be applied in order.

  ```
  # foo-00: initial spreadsheet setup
  a1 = 1
  a2 = 2
  a3 = 3

  b1 = SUM(a1:a3)
  b2 = MIN(a1:a3)
  b3 = b1 + b2
  # evaluate [a1, a2, a3] -> [b1, b2] -> b3
  ```

  ```
  # foo-01: update `a1`
  a1 = 2
  # evaluate a1 -> [b1, b2] -> b3
  ```

  ```
  # foo-02: update `a2` and `a3`
  a2 = 4
  a3 = 6
  # evaluate [a2, a3] -> [b1, b2] -> b3
  ```
