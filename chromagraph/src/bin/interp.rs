use std::{fs, io, path::PathBuf};

use chromagraph::*;
use clap::Parser;

/// Interprets the given Chromagraph program, writing the resulting image to the given filename
#[derive(Parser, Clone)]
struct InterpArgs {
    /// The Chromagraph program to interpret
    program: PathBuf,
    /// The output rendered image in PPM format
    #[arg(default_value = "out.ppm")]
    output: PathBuf,
    /// The width of the rendered image
    #[arg(short = 'W', long, default_value_t = 1024)]
    width: usize,
    /// The height of the rendered image
    #[arg(short = 'H', long, default_value_t = 1024)]
    height: usize,
}

fn main() -> io::Result<()> {
    let args = InterpArgs::parse();
    let mut image = Image::new(args.width, args.height);
    let aspect = args.width as f32 / args.height as f32;

    let prgm = parse(&fs::read_to_string(args.program)?).unwrap();

    for row in 0..args.height {
        // flip y axis
        let y = conv_dim(args.height - row - 1, args.height);
        for col in 0..args.width {
            let x = conv_dim(col, args.width) * aspect;
            image[(row, col)] = interp(&prgm, x, y, aspect);
        }
    }

    image.write_ppm(fs::File::create(args.output)?)?;

    Ok(())
}

/// Converts a value in range `[0, dim - 1]` to a f32 in `[-1, 1]`
fn conv_dim(value: usize, dim: usize) -> f32 {
    value as f32 / (dim - 1) as f32 * 2. - 1.
}

/// Interprets the given [`Chromagraph`] program for the given `x`, `y`, and `aspect`
fn interp(prgm: &Chromagraph, x: f32, y: f32, aspect: f32) -> [f32; 3] {
    use Value::{Bool, Float};

    let mut values: Vec<Value> = Vec::with_capacity(prgm.exprs.len());

    for expr in &prgm.exprs {
        values.push(match expr {
            Expr::X => Float(x),
            Expr::Y => Float(y),
            Expr::Aspect => Float(aspect),
            Expr::Const(c) => Float(*c),
            Expr::Unary(UnaryOp::Neg, v) => Float(-values[v.0].f()),
            Expr::Unary(UnaryOp::Abs, v) => Float(values[v.0].f().abs()),
            Expr::Unary(UnaryOp::Sqrt, v) => Float(values[v.0].f().sqrt()),
            Expr::Unary(UnaryOp::Ipart, v) => Float(values[v.0].f().fract()),
            Expr::Unary(UnaryOp::Fpart, v) => Float(values[v.0].f().trunc()),
            Expr::Binary(BinaryOp::Add, l, r) => Float(values[l.0].f() + values[r.0].f()),
            Expr::Binary(BinaryOp::Sub, l, r) => Float(values[l.0].f() - values[r.0].f()),
            Expr::Binary(BinaryOp::Mul, l, r) => Float(values[l.0].f() * values[r.0].f()),
            Expr::Binary(BinaryOp::Div, l, r) => Float(values[l.0].f() / values[r.0].f()),
            Expr::Binary(BinaryOp::Min, l, r) => Float(f32::min(values[l.0].f(), values[r.0].f())),
            Expr::Binary(BinaryOp::Max, l, r) => Float(f32::max(values[l.0].f(), values[r.0].f())),
            Expr::Lt(l, r) => Bool(values[l.0].f() < values[r.0].f()),
            Expr::Leq(l, r) => Bool(values[l.0].f() <= values[r.0].f()),
            Expr::Eq(l, r) => Bool(values[l.0].f() == values[r.0].f()),
            Expr::Not(v) => Bool(!values[v.0].b()),
            Expr::And(l, r) => Bool(values[l.0].b() && values[r.0].b()),
            Expr::Or(l, r) => Bool(values[l.0].b() || values[r.0].b()),
            Expr::Select {
                cond,
                if_true,
                if_false,
            } => {
                if values[cond.0].b() {
                    Float(values[if_true.0].f())
                } else {
                    Float(values[if_false.0].f())
                }
            }
        });
    }

    [
        values[prgm.ret[0].0].f(),
        values[prgm.ret[1].0].f(),
        values[prgm.ret[2].0].f(),
    ]
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Float(f32),
    Bool(bool),
}

impl Value {
    fn f(self) -> f32 {
        let Self::Float(f) = self else {
            panic!("trying to unwrap a non-float value as a float");
        };
        f
    }

    fn b(self) -> bool {
        let Self::Bool(b) = self else {
            panic!("trying to unwrap a non-bool value as a bool");
        };
        b
    }
}
