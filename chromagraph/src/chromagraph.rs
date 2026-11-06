use std::fmt;

/// A unique ID for a value and its [`Expr`], which is its index in the list of [`Chromagraph`] expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueId(usize);

impl From<ValueId> for usize {
    fn from(value: ValueId) -> Self {
        value.0
    }
}

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

/// Utility for building a [`Chromagraph`] from [`Expr`]s
#[derive(Default, Clone)]
pub struct ChromagraphBuilder {
    exprs: Vec<Expr>,
}

impl ChromagraphBuilder {
    /// Pushes an [`Expr`] and returns its new [`ValueId`]
    #[must_use]
    pub fn push(&mut self, expr: Expr) -> ValueId {
        self.exprs.push(expr);
        ValueId(self.exprs.len() - 1)
    }

    /// Consumes self to build a [`Chromagraph`] returning the given values
    #[must_use]
    pub fn ret(self, ret: [ValueId; 3]) -> Chromagraph {
        Chromagraph {
            exprs: self.exprs,
            ret,
        }
    }
}
