# Regular-Expression Compiler

Compile a small regular-expression syntax into a matcher and show how compiled matching outperforms naive backtracking.

## First pass

Implement literals, concatenation, alternation, grouping, `*`, `+`, `?`, and character classes.
Interpret the expression with a straightforward backtracking matcher or an AST evaluator.

## Compiler extension

Compile expressions into a Thompson NFA bytecode.
Optionally determinize small automata or generate specialized native code for literal-heavy patterns.
Track source spans so a match can be highlighted in the input.

## Evidence of success

Visualize the automaton as Graphviz output.
Compare the naive matcher and NFA engine on a pathological expression and a log-search workload.
Print match locations and capture-free match results.
