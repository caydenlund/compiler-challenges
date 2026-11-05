# Image Kernel Language

Parse a small language for image transformations and compile it into fast image-processing code.

```
// outputs a white circle on a black background
// usage: `circle [dims]`
args(dims: int)

fn is_in_circle(x: float, y: float) -> bool {
  x * x + y * y <= 1
}

fn color(row: int, col: int, dims: int) -> float[3] {
  // col in [0, dims-1] => x in [-1, 1]
  let x = 2.0 * float(col) / float(dims - 1) - 1.0;
  // row in [0, dims-1] => y in [-1, 1] (flipped)
  let y = 1.0 - 2.0 * float(row) / float(dims - 1);
  if is_in_circle(x, y) then [1.0, 1.0, 1.0] else [0.0, 0.0, 0.0]
}

return array[row: dims, col: dims] color(row, col, dims)
```

```
// does a 3x3 blur of the input image
// usage: `blur [input]`
args(input: string)

fn blur(src: float[h, w, 3], row: int, col: int) -> float[3] {
  let col_start = if col == 0 then 0 else col - 1
  let col_end = if col == w - 1 then w - 1 else col + 1
  let cols = col_end - col_start + 1

  let row_start = if row == 0 then 0 else row - 1
  let row_end = if row == h - 1 then h - 1 else row + 1
  let rows = row_end - row_start + 1

  let r = sum[rowi: rows, coli: cols] pixels[row_start + rowi, col_start + coli, 0]
  let g = sum[rowi: rows, coli: cols] pixels[row_start + rowi, col_start + coli, 1]
  let b = sum[rowi: rows, coli: cols] pixels[row_start + rowi, col_start + coli, 2]
  let cells = float(rows * cols)
  [r / cells, g / cells, b / cells]
}

load_img input to src[h, w]
return array[row: h, col: w] blur(src, row, col)
```

```
// rotates the input image clockwise
// usage: `blur [input]`
args(input: string)

load_img input to src[in_h, in_w]
return array[out_row: in_w, out_col: in_h] src[in_h - 1 - out_col, out_row]
```


## First pass

Interpret the language.
Use PPM RGB for input and output.


## Compiler extension

Compile each kernel to native code, SIMD-friendly bytecode, or generated C.
Hoist invariant expressions and recognize simple pointwise operations.
Optionally process rows in parallel without changing output pixels.


## Evidence of success

Produce before-and-after images for various benchmarks.
Time the interpreter and compiled kernel on a megapixel image.
Display the kernel source next to the output image and benchmark.
