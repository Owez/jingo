//! Expression-centric abstract syntax tree for Jingo

use std::ops::Range;

/// Central expression structure, defining the fundamental structure of Jingo
///
/// To parse into this structure and therefore an [ExprKind], please use the
/// [Parse](crate::frontend::parser::Parse) trait.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    /// Kind/variant of this expression, this contains the underlying main data
    /// for an expression
    kind: ExprKind,

    /// Optional documentation string
    doc: Option<String>,
}

/// Expression kind enumeration for the AST, containing all possible variants for
/// the AST to use, stemming from the central [Expr] structure
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    BinOp(BinOp),
    Class(Class),
    Function(Function),
    Method(Method),
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    If(If),
    While(While),
    Return(Return),
    Variable(Variable),
    SetVariable(SetVariable),
    IntLit(IntLit),
    FloatLit(FloatLit),
    StringLit(StringLit),
    CharLit(CharLit),
}

/// Binary operation varients, defining allowed types of a [BinOp] expression
#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    EqEq,
    NotEq,
    And,
    Or,
    PlusEq,
    SubEq,
}

/// Binary operation allowing two [ExprKind]s to be modified by a mathematical notation
#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    /// Leftmost expression
    pub left: Box<Expr>,

    /// Rightmost expression
    pub right: Box<Expr>,

    /// Mathematical notation to modify [BinOp::left] and [BinOp::right] together by
    pub kind: BinOpKind,
}

/// Pre-validated valid identifier
#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    /// Actual identifier name/data programmer passed
    pub inner: String,

    /// Positional data
    pub range: Range<usize>,
}

/// Class definition
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    /// Name of class
    pub name: Id,

    /// Start ind
    pub start: usize,
}

/// Subprogram allowing code modularity, recurses down into more [ExprKind]
/// nodes. This is different from the [Method] structure as this one is for
/// non-class-linked subprograms
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// Identifier of the function
    pub id: Id,

    /// Allowed arguments to be passed
    pub args: Vec<String>,

    /// Body of function
    pub body: Vec<ExprKind>,

    /// Start ind
    pub start: usize,
}

/// Class-linked subprogram similar to the base [Function], but is strictly linked
/// to a certain class
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    /// Reference to the class name (which should be an existing [Class]) the
    /// method is linked to
    pub class_id: Id,

    /// Identifier of the method
    pub id: Id,

    /// Allowed arguments to be passed
    pub args: Vec<Id>,

    /// Body of method
    pub body: Vec<ExprKind>,

    /// Distinguishes between a creation method (defined with `::`) or a normal
    /// method (defined with `.`)
    pub creation_method: bool,

    /// Start ind
    pub start: usize,
}

/// Caller for a function, allows invoking functions with passed arguments
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// Identifier of the function ([Id::range.start] should be used as the start)
    pub id: Id,

    /// Argument to pass and invoke within the function
    pub args: Vec<ExprKind>,
}

/// Caller for a method, allows invoking methods with passed arguments
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall {
    /// Reference to the class name (which should be an existing [Class]) the
    /// method is linked to
    pub class_id: Id,

    /// Identifier of the function ([Id::range.start] should be used as the start)
    pub id: Id,

    /// Argument to pass and invoke within the function
    pub args: Vec<ExprKind>,
}

/// Basic single-argument matching as part of a broader [If]
#[derive(Debug, Clone, PartialEq)]
pub struct IfSegment {
    /// Condition needed in order to fire
    pub condition: ExprKind,

    /// Body of if
    pub body: Vec<ExprKind>,

    /// Start ind
    pub start: usize,
}

/// Default value for [If] statement, typically known as `else`
#[derive(Debug, Clone, PartialEq)]
pub struct IfDefault {
    /// Body of if default
    pub body: Vec<ExprKind>,

    /// Start ind
    pub start: usize,
}

/// Broader structure for basic single-argument matching
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    /// Arranged as `if, else if, else if`
    pub segments: Vec<IfSegment>,

    /// Optional final `else`
    pub default: Option<IfDefault>,
}

/// While loop, requiring a condition in order to fire the body repeatedly
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    /// Condition needed in order to fire
    pub condition: Box<Expr>,

    /// Body of while
    pub body: Vec<ExprKind>,

    /// Start ind
    pub start: usize,
}

/// Return expression allowing pass-back from functions
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    /// Expression to return result from
    pub expr: Box<Expr>,

    /// Start ind
    pub start: usize,
}

/// Variable definition, allowing reusability & reference to given data, this
/// structure defines the initial variable state which may be change if
/// [Variable::mutable] is [true]
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    /// Determines if this variable is mutable
    pub mutable: bool,

    /// Variable identifier
    pub id: Id,

    /// Expression which determines initial variable state
    pub expr: Box<Expr>,

    /// Start ind
    pub start: usize,
}

/// Variable setter for overwriting data in an existing [Variable] whilst
/// [Variable::mutable] is [true]
#[derive(Debug, Clone, PartialEq)]
pub struct SetVariable {
    /// Variable identifier ([Id::range.start] should be used as the start)
    pub id: Id,

    /// Expression determining what [SetVariable::id] should be set to
    pub expr: Box<Expr>,
}

/// Integer literal used for defining raw integers
#[derive(Debug, Clone, PartialEq)]
pub struct IntLit {
    /// Actual integer used
    pub inner: i64,

    /// Positional data
    pub range: Range<usize>,
}

/// Float literal used for defining raw floats
#[derive(Debug, Clone, PartialEq)]
pub struct FloatLit {
    /// Actual float used
    pub inner: f64,

    /// Positional data
    pub range: Range<usize>,
}

/// String literal used for defining raw strings
#[derive(Debug, Clone, PartialEq)]
pub struct StringLit {
    /// Actual string used
    pub inner: String,

    /// Positional data
    pub range: Range<usize>,
}

/// Char literal used for defining raw chars
#[derive(Debug, Clone, PartialEq)]
pub struct CharLit {
    /// Actual char used
    pub inner: char,

    /// Positional data
    pub range: Range<usize>,
}
