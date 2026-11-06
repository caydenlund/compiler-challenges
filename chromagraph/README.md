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

f_x = x
f_y = y
f_aspect = aspect

c_0 = const 0.0
c_1 = const 1.0

f_x2 = mul f_x f_x
f_y2 = mul f_y f_y
f_dist_sq = add f_x2 f_y2

f_radius = min f_aspect c_1
f_radius_sq = mul f_radius f_radius
b_in_square = leq f_dist_sq f_radius_sq

f_color = select b_in_square c_1 c_0

return f_color f_color f_color
```

If you're having trouble visualizing what these primitives mean, here's a more readable representation of the same computation:

```rust
fn circle(x: f32, y: f32, aspect: f32) -> [f32; 3] {
    let dist_sq = (x * x) + (y * y);

    let radius = f32::min(aspect, 1.0);
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
        # () -> f32
        if op == "x": values.append(x)
        elif op == "y": values.append(y)
        elif op == "aspect": values.append(aspect)
        elif op == "const": values.append(args[0])
        # f32 -> f32
        elif op == "neg": values.append(-get(0))
        elif op == "abs": values.append(abs(get(0)))
        elif op == "sqrt": values.append(math.sqrt(get(0)))
        elif op == "ipart": values.append(math.modf(get(0))[1])
        elif op == "fpart": values.append(math.modf(get(0))[0])
        # (f32, f32) -> f32
        elif op == "add": values.append(get(0) + get(1))
        elif op == "sub": values.append(get(0) - get(1))
        elif op == "mul": values.append(get(0) * get(1))
        elif op == "div": values.append(get(0) / get(1))
        elif op == "min": values.append(min(get(0), get(1)))
        elif op == "max": values.append(max(get(0), get(1)))
        # (f32, f32) -> bool
        elif op == "lt": values.append(get(0) < get(1))
        elif op == "leq": values.append(get(0) <= get(1))
        elif op == "eq": values.append(get(0) == get(1))
        # logic
        elif op == "not": values.append(not get(0))
        elif op == "and": values.append(get(0) and get(1))
        elif op == "or": values.append(get(0) or get(1))
        elif op == "select": values.append(get(1) if get(0) else get(2))
        else: raise ValueError("unknown op: '" + op + "'")
    return [values[index] for index in ret]
```

A more fully-featured interpreter in Rust, including more sophisticated error handling and actual image generation, can be found in the [`src/` directory](./src).

Instead of control flow in the programs, all branches are eagerly executed and the result is chosen afterwards with the `select` instruction.
Can you write an interpreter that identifies subexpressions that are only used conditionally?
What other optimizations can you make?
Can you write a JIT or an AOT compiler?
Does a heavy backend like LLVM provide a net benefit in a JIT context?
What about Cranelift?

## Benchmarks

All benchmarks can be found in the `benches/` directory, alongside a sample render in 512x512 dimensions.
They vary in complexity, roughly in the following order:

- `gradient`: draws a gradient depending only on `x/aspect` and `y`
- `circle`: draws a white circle on a black background
- `simple-shapes`: draws a square, a circle, a triangle, and a hexagon
- `graph`: plots `y = x - x^3 / 6 + x^5 / 120 - x^7 / 5040`
