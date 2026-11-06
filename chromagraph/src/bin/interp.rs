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
        let val_float = |id: &ValueId| values[usize::from(*id)].as_float();
        let val_bool = |id: &ValueId| values[usize::from(*id)].as_bool();

        let v = match expr {
            Expr::X => Float(x),
            Expr::Y => Float(y),
            Expr::Aspect => Float(aspect),
            Expr::Const(c) => Float(*c),
            Expr::Unary(UnaryOp::Neg, v) => Float(-val_float(v)),
            Expr::Unary(UnaryOp::Abs, v) => Float(val_float(v).abs()),
            Expr::Unary(UnaryOp::Sqrt, v) => Float(val_float(v).sqrt()),
            Expr::Unary(UnaryOp::Ipart, v) => Float(val_float(v).trunc()),
            Expr::Unary(UnaryOp::Fpart, v) => Float(val_float(v).fract()),
            Expr::Binary(BinaryOp::Add, l, r) => Float(val_float(l) + val_float(r)),
            Expr::Binary(BinaryOp::Sub, l, r) => Float(val_float(l) - val_float(r)),
            Expr::Binary(BinaryOp::Mul, l, r) => Float(val_float(l) * val_float(r)),
            Expr::Binary(BinaryOp::Div, l, r) => Float(val_float(l) / val_float(r)),
            Expr::Binary(BinaryOp::Min, l, r) => Float(f32::min(val_float(l), val_float(r))),
            Expr::Binary(BinaryOp::Max, l, r) => Float(f32::max(val_float(l), val_float(r))),
            Expr::Lt(l, r) => Bool(val_float(l) < val_float(r)),
            Expr::Leq(l, r) => Bool(val_float(l) <= val_float(r)),
            Expr::Eq(l, r) => Bool(val_float(l) == val_float(r)),
            Expr::Not(v) => Bool(!val_bool(v)),
            Expr::And(l, r) => Bool(val_bool(l) && val_bool(r)),
            Expr::Or(l, r) => Bool(val_bool(l) || val_bool(r)),
            Expr::Select {
                cond,
                if_true,
                if_false,
            } => {
                if val_bool(cond) {
                    Float(val_float(if_true))
                } else {
                    Float(val_float(if_false))
                }
            }
        };

        values.push(v);
    }

    [
        values[usize::from(prgm.ret[0])].as_float(),
        values[usize::from(prgm.ret[1])].as_float(),
        values[usize::from(prgm.ret[2])].as_float(),
    ]
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Float(f32),
    Bool(bool),
}

impl Value {
    fn as_float(self) -> f32 {
        let Self::Float(f) = self else {
            panic!("trying to unwrap a non-float value as a float");
        };
        f
    }

    fn as_bool(self) -> bool {
        let Self::Bool(b) = self else {
            panic!("trying to unwrap a non-bool value as a bool");
        };
        b
    }
}
