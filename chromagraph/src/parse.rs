use std::{collections::HashMap, fmt};

/// A unique identifier for a value produced by an [`Expr`],
/// which is its index in the list of [`Chromagraph`] expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueId(pub usize);

/// The type of a value produced by an [`Expr`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    /// A floating-point value
    Float,
    /// A boolean value
    Bool,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float => f.write_str("float"),
            Self::Bool => f.write_str("bool"),
        }
    }
}

/// An expression in a [`Chromagraph`], producing a value uniquely identified by a [`ValueId`]
#[derive(Debug, Clone)]
pub enum Expr {
    // () -> float
    /// The `x` value of the currently-evaluated pixel, in range `[-aspect, aspect]`
    X,
    /// The `y` value of the currently-evaluated pixel, in range `[-1, 1]`
    Y,
    /// The aspect ratio of the image, as `width / height`
    Aspect,
    /// A literal floating-point constant
    Const(f32),

    /// A unary operation: float -> float
    Unary(UnaryOp, ValueId),

    /// A binary operation: (float, float) -> float
    Binary(BinaryOp, ValueId, ValueId),

    // (float, float) -> bool
    /// True when v0 < v1
    Lt(ValueId, ValueId),
    /// True when v0 <= v1
    Leq(ValueId, ValueId),
    /// True when v0 == v1
    Eq(ValueId, ValueId),

    // logic
    /// Logical not of the given boolean value
    Not(ValueId),
    /// True when both given boolean values are true
    And(ValueId, ValueId),
    /// True when at least one of the given boolean values is true
    Or(ValueId, ValueId),
    /// Conditionally select (float) one or another value based on (bool) predicate
    Select {
        cond: ValueId,
        if_true: ValueId,
        if_false: ValueId,
    },
}

/// An enumeration of the various unary operations on floating-point values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Negate the given value
    Neg,
    /// Absolute value of the given value
    Abs,
    /// Square root of the given value
    Sqrt,
    /// Integer part of the given value
    Ipart,
    /// Fractional part of the given value
    Fpart,
}

/// An enumeration of the various binary operations on floating-point values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Add the given values
    Add,
    /// Subtract the given values
    Sub,
    /// Multiply the given values
    Mul,
    /// Divide the given values
    Div,
    /// Lesser of the given values
    Min,
    /// Greater of the given values
    Max,
}

/// A `Chromagraph` program, consisting of a series of value-producing expressions and returning a single color
#[derive(Clone)]
pub struct Chromagraph {
    /// The value-producing expressions
    pub exprs: Vec<Expr>,
    /// The [`ValueId`]s of the 3 values to return, respectively referring to the red, green, and blue color channels
    pub ret: [ValueId; 3],
}

/// Parses the given text into a [`Chromagraph`] program
///
/// # Errors
///
/// Returns a [`ParseError`] if the input text is malformed
pub fn parse(text: &str) -> Result<Chromagraph, ParseError> {
    fn get_value(
        values: &HashMap<String, (ValueId, ValueType)>,
        name: &str,
        line: &str,
    ) -> Result<(ValueId, ValueType), ParseError> {
        values
            .get(name)
            .copied()
            .ok_or_else(|| ParseError::UndefinedValue {
                value: name.into(),
                line: line.into(),
            })
    }
    let mut values = HashMap::new();
    let mut lines = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        let words: Vec<_> = line.split_whitespace().collect();
        if words.is_empty() || words[0].starts_with('#') {
            continue;
        }
        if words[0] == "return" {
            let (ret, ret_types) = words[1..].iter().try_fold(
                (Vec::new(), Vec::new()),
                |(mut ret, mut ret_types), &word| {
                    let (value, value_type) = get_value(&values, word, line)?;
                    ret.push(value);
                    ret_types.push(value_type);
                    Ok((ret, ret_types))
                },
            )?;
            let expected = vec![ValueType::Float; 3];
            check_arg_types("return", &expected, &ret_types, line)?;
            let ret = [ret[0], ret[1], ret[2]];
            return Ok(Chromagraph { exprs: lines, ret });
        }
        if words.len() < 3 || words[1] != "=" {
            return Err(ParseError::MalformedLine { line: line.into() });
        }
        let op = words[2];
        let expected = arg_types_for(op).ok_or_else(|| ParseError::UnknownOp {
            op: op.into(),
            line: line.into(),
        })?;
        let (arg_vals, arg_types) = if op == "const" {
            (Vec::new(), vec![ValueType::Float; words[3..].len()])
        } else {
            words[3..].iter().try_fold(
                (Vec::new(), Vec::new()),
                |(mut arg_vals, mut arg_types), &word| {
                    let (value, value_type) = get_value(&values, word, line)?;
                    arg_vals.push(value);
                    arg_types.push(value_type);
                    Ok((arg_vals, arg_types))
                },
            )?
        };
        check_arg_types(op, expected, &arg_types, line)?;

        let expr = match op {
            "x" => Expr::X,
            "y" => Expr::Y,
            "aspect" => Expr::Aspect,
            "const" => Expr::Const(
                words[3]
                    .parse()
                    .map_err(|_| ParseError::MalformedLine { line: line.into() })?,
            ),
            "neg" => Expr::Unary(UnaryOp::Neg, arg_vals[0]),
            "abs" => Expr::Unary(UnaryOp::Abs, arg_vals[0]),
            "sqrt" => Expr::Unary(UnaryOp::Sqrt, arg_vals[0]),
            "ipart" => Expr::Unary(UnaryOp::Ipart, arg_vals[0]),
            "fpart" => Expr::Unary(UnaryOp::Fpart, arg_vals[0]),
            "add" => Expr::Binary(BinaryOp::Add, arg_vals[0], arg_vals[1]),
            "sub" => Expr::Binary(BinaryOp::Sub, arg_vals[0], arg_vals[1]),
            "mul" => Expr::Binary(BinaryOp::Mul, arg_vals[0], arg_vals[1]),
            "div" => Expr::Binary(BinaryOp::Div, arg_vals[0], arg_vals[1]),
            "min" => Expr::Binary(BinaryOp::Min, arg_vals[0], arg_vals[1]),
            "max" => Expr::Binary(BinaryOp::Max, arg_vals[0], arg_vals[1]),
            "lt" => Expr::Lt(arg_vals[0], arg_vals[1]),
            "leq" => Expr::Leq(arg_vals[0], arg_vals[1]),
            "eq" => Expr::Eq(arg_vals[0], arg_vals[1]),
            "not" => Expr::Not(arg_vals[0]),
            "and" => Expr::And(arg_vals[0], arg_vals[1]),
            "or" => Expr::Or(arg_vals[0], arg_vals[1]),
            "select" => Expr::Select {
                cond: arg_vals[0],
                if_true: arg_vals[1],
                if_false: arg_vals[2],
            },
            _ => unreachable!("operations are checked above"),
        };
        let value_type = result_type_for(op);
        let value = ValueId(lines.len());
        lines.push(expr);
        values.insert(words[0].into(), (value, value_type));
    }

    Err(ParseError::MissingReturn)
}

fn arg_types_for(op: &str) -> Option<&'static [ValueType]> {
    use ValueType::{Bool, Float};

    match op {
        "x" | "y" | "aspect" => Some(&[]),
        "const" | "neg" | "abs" | "sqrt" | "ipart" | "fpart" => Some(&[Float]),
        "add" | "sub" | "mul" | "div" | "min" | "max" | "lt" | "leq" | "eq" => {
            Some(&[Float, Float])
        }
        "not" => Some(&[Bool]),
        "and" | "or" => Some(&[Bool, Bool]),
        "select" => Some(&[Bool, Float, Float]),
        _ => None,
    }
}

fn result_type_for(op: &str) -> ValueType {
    match op {
        "lt" | "leq" | "eq" | "not" | "and" | "or" => ValueType::Bool,
        _ => ValueType::Float,
    }
}

fn check_arg_types(
    op: &str,
    expected: &[ValueType],
    actual: &[ValueType],
    line: &str,
) -> Result<(), ParseError> {
    if expected == actual {
        return Ok(());
    }

    Err(ParseError::WrongArgTypes {
        op: op.into(),
        expected: expected.into(),
        actual: actual.into(),
        line: line.into(),
    })
}

/// An error raised by parsing malformed Chromagraph text
#[derive(Debug, Clone)]
pub enum ParseError {
    /// A referred-to value is undefined
    UndefinedValue {
        /// The undefined value
        value: String,
        /// The text of the line referring to an undefined value
        line: String,
    },
    /// An undefined operation is used
    UnknownOp {
        /// The undefined operation
        op: String,
        /// The text of the line using an undefined operation
        line: String,
    },
    /// A line in the Chromagraph text is malformed
    MalformedLine {
        /// The text of the malformed line
        line: String,
    },
    /// An operation has arguments with an invalid type signature (including wrong number of args)
    WrongArgTypes {
        /// The operation in question
        op: String,
        /// The expected types, in argument order
        expected: Vec<ValueType>,
        /// The actual types, in argument order
        actual: Vec<ValueType>,
        /// The text of the line using the invalid arguments
        line: String,
    },
    MissingReturn,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedValue { value, line } => {
                write!(f, "undefined value '{value}' in line '{line}'")
            }
            Self::UnknownOp { op, line } => {
                write!(f, "unknown operation '{op}' in line '{line}'")
            }
            Self::MalformedLine { line } => write!(f, "malformed line: '{line}'"),
            Self::WrongArgTypes {
                op,
                expected,
                actual,
                line,
            } => {
                let expected = expected
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                let actual = actual
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "wrong argument types ([{actual}] instead of [{expected}]) for operation '{op}' in line '{line}'"
                )
            }
            Self::MissingReturn => write!(f, "missing `return` line"),
        }
    }
}
