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
    doc: String,

    /// Character range used for this expression
    range: Range<usize>,
}

impl Expr {
    /// Shortcut method for getting from parsing
    pub(crate) fn from_parse(
        kind: impl Into<ExprKind>,
        doc: &mut Vec<String>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind: kind.into(),
            doc: doc.join("\n"),
            range: start..end,
        }
    }
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

/// Binary operation variants, defining allowed types of a [BinOp] expression
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

/// Binary operation allowing two [Expr]s to be modified by a mathematical notation
#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    /// Leftmost expression
    pub left: Box<Expr>,

    /// Rightmost expression
    pub right: Box<Expr>,

    /// Mathematical notation to modify [BinOp::left] and [BinOp::right] together by
    pub kind: BinOpKind,
}

impl From<BinOp> for ExprKind {
    fn from(kind: BinOp) -> ExprKind {
        ExprKind::BinOp(kind)
    }
}

/// Pre-validated valid identifier
#[derive(Debug, Clone, PartialEq)]
pub struct Id(pub String);

/// Class definition
#[derive(Debug, Clone, PartialEq)]
pub struct Class(pub Id);

impl From<Class> for ExprKind {
    fn from(kind: Class) -> ExprKind {
        ExprKind::Class(kind)
    }
}

/// Subprogram allowing code modularity, recurses down into more [Expr]
/// nodes. This is different from the [Method] structure as this one is for
/// non-class-linked subprograms
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// Identifier of the function
    pub id: Id,

    /// Allowed arguments to be passed
    pub args: Vec<Id>,

    /// Body of function
    pub body: Vec<Expr>,
}

impl From<Function> for ExprKind {
    fn from(kind: Function) -> ExprKind {
        ExprKind::Function(kind)
    }
}

/// Class-linked subprogram similar to the base [Function], but is strictly linked
/// to a certain class
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    /// Reference to the class name (which should be an existing [Class]) the
    /// method is linked to
    pub class_id: Id,

    /// Distinguishes between a creation method (defined with `::`) or a normal
    /// method (defined with `.`)
    pub creation_method: bool,

    /// Identifier of the method
    pub id: Id,

    /// Allowed arguments to be passed
    pub args: Vec<Id>,

    /// Body of method
    pub body: Vec<Expr>,
}

impl From<Method> for ExprKind {
    fn from(kind: Method) -> ExprKind {
        ExprKind::Method(kind)
    }
}

/// Caller for a function, allows invoking functions with passed arguments
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// Identifier of the function ([Id::range.start] should be used as the start)
    pub id: Id,

    /// Argument to pass and invoke within the function
    pub args: Vec<Expr>,
}

impl From<FunctionCall> for ExprKind {
    fn from(kind: FunctionCall) -> ExprKind {
        ExprKind::FunctionCall(kind)
    }
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
    pub args: Vec<Expr>,
}

impl From<MethodCall> for ExprKind {
    fn from(kind: MethodCall) -> ExprKind {
        ExprKind::MethodCall(kind)
    }
}

/// Basic single-argument matching as part of a broader [If]
#[derive(Debug, Clone, PartialEq)]
pub struct IfSegment {
    /// Condition needed in order to fire
    pub condition: Expr,

    /// Body of if
    pub body: Vec<Expr>,
}

/// Default value for [If] statement, typically known as `else`
#[derive(Debug, Clone, PartialEq)]
pub struct IfDefault(Vec<Expr>);

/// Broader structure for basic single-argument matching
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    /// Arranged as `if, else if, else if`
    pub segments: Vec<IfSegment>,

    /// Optional final `else`
    pub default: Option<IfDefault>,
}

impl From<If> for ExprKind {
    fn from(kind: If) -> ExprKind {
        ExprKind::If(kind)
    }
}

/// While loop, requiring a condition in order to fire the body repeatedly
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    /// Condition needed in order to fire
    pub condition: Box<Expr>,

    /// Body of while
    pub body: Vec<Expr>,
}

impl From<While> for ExprKind {
    fn from(kind: While) -> ExprKind {
        ExprKind::While(kind)
    }
}

/// Return expression allowing pass-back from functions
#[derive(Debug, Clone, PartialEq)]
pub struct Return(pub Box<Expr>);

impl From<Return> for ExprKind {
    fn from(kind: Return) -> ExprKind {
        ExprKind::Return(kind)
    }
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
}

impl From<Variable> for ExprKind {
    fn from(kind: Variable) -> ExprKind {
        ExprKind::Variable(kind)
    }
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

impl From<SetVariable> for ExprKind {
    fn from(kind: SetVariable) -> ExprKind {
        ExprKind::SetVariable(kind)
    }
}

/// Integer literal used for defining raw integers
#[derive(Debug, Clone, PartialEq)]
pub struct IntLit(pub i64);

impl From<IntLit> for ExprKind {
    fn from(kind: IntLit) -> ExprKind {
        ExprKind::IntLit(kind)
    }
}

/// Float literal used for defining raw floats
#[derive(Debug, Clone, PartialEq)]
pub struct FloatLit(pub f64);

impl From<FloatLit> for ExprKind {
    fn from(kind: FloatLit) -> ExprKind {
        ExprKind::FloatLit(kind)
    }
}

/// String literal used for defining raw strings
#[derive(Debug, Clone, PartialEq)]
pub struct StringLit(pub String);

impl From<StringLit> for ExprKind {
    fn from(kind: StringLit) -> ExprKind {
        ExprKind::StringLit(kind)
    }
}

/// Char literal used for defining raw chars
#[derive(Debug, Clone, PartialEq)]
pub struct CharLit(pub char);

impl From<CharLit> for ExprKind {
    fn from(kind: CharLit) -> ExprKind {
        ExprKind::CharLit(kind)
    }
}
