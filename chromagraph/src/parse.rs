use std::{collections::HashMap, fmt};

use crate::{BinaryOp, Chromagraph, ChromagraphBuilder, Expr, UnaryOp, ValueId, ValueType};

type ParseResult<T> = Result<T, ParseError>;

/// Parses the given text into a [`Chromagraph`] program
///
/// # Errors
///
/// Returns a [`ParseError`] if the input text is malformed
pub fn parse(text: &str) -> ParseResult<Chromagraph> {
    let mut values: HashMap<String, (ValueId, ValueType)> = HashMap::new();
    let mut prgm = ChromagraphBuilder::default();

    for line in text.lines() {
        let line = line.trim();
        let words: Vec<_> = line.split_whitespace().collect();
        if words.is_empty() || words[0].starts_with('#') {
            continue;
        }

        let get_value = |name: &str| -> ParseResult<(ValueId, ValueType)> {
            values
                .get(name)
                .copied()
                .ok_or_else(|| ParseError::UndefinedValue {
                    value: name.into(),
                    line: line.into(),
                })
        };

        if words[0] == "return" {
            let (ret, ret_types) = words[1..].iter().try_fold(
                (Vec::new(), Vec::new()),
                |(mut ret, mut ret_types), &word| {
                    let (value, value_type) = get_value(word)?;
                    ret.push(value);
                    ret_types.push(value_type);
                    Ok((ret, ret_types))
                },
            )?;
            let expected = vec![ValueType::Float; 3];
            check_arg_types("return", &expected, &ret_types, line)?;
            return Ok(prgm.ret([ret[0], ret[1], ret[2]]));
        }

        // expected form: `<value_id> = <op> [args...]`
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
                    let (value, value_type) = get_value(word)?;
                    arg_vals.push(value);
                    arg_types.push(value_type);
                    Ok((arg_vals, arg_types))
                },
            )?
        };
        check_arg_types(op, expected, &arg_types, line)?;
        let vtype = result_type_for(op);

        let value = {
            use BinaryOp::{Add, Div, Max, Min, Mul, Sub};
            use Expr::{And, Aspect, Binary, Const, Eq, Leq, Lt, Not, Or, Select, Unary, X, Y};
            use UnaryOp::{Abs, Fpart, Ipart, Neg, Sqrt};
            let expr = match op {
                "x" => X,
                "y" => Y,
                "aspect" => Aspect,
                "const" => Const(
                    words[3]
                        .parse()
                        .map_err(|_| ParseError::MalformedLine { line: line.into() })?,
                ),
                "neg" => Unary(Neg, arg_vals[0]),
                "abs" => Unary(Abs, arg_vals[0]),
                "sqrt" => Unary(Sqrt, arg_vals[0]),
                "ipart" => Unary(Ipart, arg_vals[0]),
                "fpart" => Unary(Fpart, arg_vals[0]),
                "add" => Binary(Add, arg_vals[0], arg_vals[1]),
                "sub" => Binary(Sub, arg_vals[0], arg_vals[1]),
                "mul" => Binary(Mul, arg_vals[0], arg_vals[1]),
                "div" => Binary(Div, arg_vals[0], arg_vals[1]),
                "min" => Binary(Min, arg_vals[0], arg_vals[1]),
                "max" => Binary(Max, arg_vals[0], arg_vals[1]),
                "lt" => Lt(arg_vals[0], arg_vals[1]),
                "leq" => Leq(arg_vals[0], arg_vals[1]),
                "eq" => Eq(arg_vals[0], arg_vals[1]),
                "not" => Not(arg_vals[0]),
                "and" => And(arg_vals[0], arg_vals[1]),
                "or" => Or(arg_vals[0], arg_vals[1]),
                "select" => Select {
                    cond: arg_vals[0],
                    if_true: arg_vals[1],
                    if_false: arg_vals[2],
                },
                _ => unreachable!("operations are checked above"),
            };
            prgm.push(expr)
        };

        values.insert(words[0].into(), (value, vtype));
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

#[inline]
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
    /// The program does not contain the required `return` statement
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
