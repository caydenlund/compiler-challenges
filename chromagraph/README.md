# The Chromagraph Challenge

Compare to [the Prospero Challenge](https://www.mattkeeter.com/projects/prospero/), but with logic and Technicolor™!

In this challenge, a Chromagraph is a program that renders an image.
Specifically, it defines a straight-line function of three inputs: `x` in `[-aspect, aspect]`, `y` in `[-1, 1]`, and `aspect = width/height`, and three outputs: `r`, `g`, and `b` in `[0, 1]`.
Your job is to evaluate this function for every pixel in an image.

The program is a series of assembly-like primitive operations with no control flow.
You can write an interpreter in an hour.

Here's a simple example:

```
# draws a white circle on a black background

v0 = x
v1 = y
v2 = aspect

# radius = min(aspect, 1)
v3 = const 1.0
v4 = min v3 v2

# x^2 + y^2
v5 = mul v0 v0
v6 = mul v1 v1
v7 = add v5 v6

# is inside circle?
v8 = mul v4 v4
v9 = leq v7 v8

# choose color: black or white
v10 = const 0.0
v11 = select v9 v3 v10

return v11 v11 v11
```

If you're having trouble visualizing what these primitives mean, here's a more readable representation of the same computation:

```rust
fn circle(x: f32, y: f32, aspect: f32) -> [f32; 3] {
    let radius = f32::min(aspect, 1.0);
    let dist_sq = (x * x) + (y * y);

    let radius_sq = radius * radius;
    if dist_sq <= radius_sq {
        [1.0, 1.0, 1.0]
    } else {
        [0.0, 0.0, 0.0]
    }
}
```

Your challenge is to execute this program, and the other chromagraphs defined here, as quickly as possible.

To start, here's a minimal interpreter in Python:

```python
import math
import sys

def parse(path):
    ops, names = [], {}
    for line in open(path):
        words = line.split()
        if not words or words[0].startswith("#"):
            continue
        if words[0] == "return":
            return ops, [names[name] for name in words[1:]]

        name, _eq, op, *args = words
        args = [names.get(arg, float(arg) if op == "const" else arg)
                for arg in args]
        names[name] = len(ops)
        ops.append((op, args))

    raise ValueError("missing return")

def run(program, x, y, aspect):
    ops, ret = program
    values = []
    for op, args in ops:
        get = lambda index: values[args[index]]
        if op == "x": values.append(x)
        elif op == "y": values.append(y)
        elif op == "aspect": values.append(aspect)
        elif op == "const": values.append(args[0])
        elif op == "neg": values.append(-get(0))
        elif op == "abs": values.append(abs(get(0)))
        elif op == "sqrt": values.append(math.sqrt(get(0)))
        elif op == "floor": values.append(math.floor(get(0)))
        elif op == "fract": values.append(math.modf(get(0))[0])
        elif op == "not": values.append(not get(0))
        elif op == "select": values.append(get(1) if get(0) else get(2))
        elif op == "add": values.append(get(0) + get(1))
        elif op == "sub": values.append(get(0) - get(1))
        elif op == "mul": values.append(get(0) * get(1))
        elif op == "div": values.append(get(0) / get(1))
        elif op == "min": values.append(min(get(0), get(1)))
        elif op == "max": values.append(max(get(0), get(1)))
        elif op == "lt": values.append(get(0) < get(1))
        elif op == "leq": values.append(get(0) <= get(1))
        elif op == "eq": values.append(get(0) == get(1))
        elif op == "and": values.append(get(0) and get(1))
        elif op == "or": values.append(get(0) or get(1))
        else: raise ValueError("unknown op: '" + op + "'")
    return [values[index] for index in ret]
```

Instead of control flow in the programs, all branches are eagerly executed and the result is chosen afterwards with the `select` instruction.
Can you write an interpreter that identifies subexpressions that are only used conditionally?
What other optimizations can you make?
Does a backend like LLVM or Cranelift provide a net benefit in a JIT context?
