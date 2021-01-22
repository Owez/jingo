//! Expression-centric abstract syntax tree for Jingo

/// Expression enumeration for the AST, the most abstract node type all other
/// AST items conform to, as everything is considered an expression
pub enum Expression {
    /// Binary operation allowing two [Expression]s to be modified by a mathmatical
    /// notation defined in [BinOp]
    BinOp((Box<Expression>, BinOp, Box<Expression>)),
}

/// Binary operation enumeration, defining allowed types of an [Expression::BinOp]
pub enum BinOp {
    /// Addition
    Add,

    /// Subtraction
    Sub,

    /// Multiplication
    Mul,

    /// Division
    Div,
}
