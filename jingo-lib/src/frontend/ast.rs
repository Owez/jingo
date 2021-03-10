//! Expression-centric abstract syntax tree for Jingo

/// Central expression structure, defining the fundamental structure of Jingo
///
/// To parse into this structure and therefore an [ExprKind], please use the
/// [Parse](crate::frontend::parser::Parse) trait.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    /// Kind/variant of this expression, this contains the underlying main data
    /// for an expression
    pub kind: ExprKind,

    /// Optional documentation string
    pub doc: Option<String>,

    /// Starting index of this expression
    pub start: usize,
}

impl Expr {
    /// Shortcut method for getting from parsing
    pub(crate) fn from_parse(kind: impl Into<ExprKind>, doc: Option<String>, start: usize) -> Self {
        Self {
            kind: kind.into(),
            doc,
            start,
        }
    }
}

/// Expression kind enumeration for the AST, containing all possible variants for
/// the AST to use, stemming from the central [Expr] structure
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Not(Not),
    Op(Op),
    Class(Class),
    Function(Function),
    Method(Method),
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    If(If),
    While(While),
    Return(Return),
    Let(Let),
    SetLet(SetLet),
    IntLit(IntLit),
    FloatLit(FloatLit),
    StrLit(StrLit),
    CharLit(CharLit),
    BoolLit(BoolLit),
}

/// Right-associative not symbol
#[derive(Debug, Clone, PartialEq)]
pub struct Not(pub Box<Expr>); // NOTE: may be replaced by general right associative for references soon

impl From<Not> for ExprKind {
    fn from(kind: Not) -> ExprKind {
        ExprKind::Not(kind)
    }
}

/// Binary operation allowing two [Expr]s to be modified by a mathematical notation
#[derive(Debug, Clone, PartialEq)]
pub struct Op {
    /// Leftmost expression
    pub left: Box<Expr>,

    /// Rightmost expression
    pub right: Box<Expr>,

    /// Mathematical notation to modify [Op::left] and [Op::right] together by
    pub kind: OpKind,
}

impl From<Op> for ExprKind {
    fn from(kind: Op) -> ExprKind {
        ExprKind::Op(kind)
    }
}

/// Binary operation variants, defining allowed types of a [Op] expression
#[derive(Debug, Clone, PartialEq)]
pub enum OpKind {
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

/// Pre-validated valid identifier
#[derive(Debug, Clone, PartialEq)]
pub struct Id(pub String);

impl From<String> for Id {
    fn from(string: String) -> Self {
        Id(string)
    }
}

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

/// Let definition, allowing reusability & reference to given data, this
/// structure defines the initial let state which may be change if
/// [Let::mutable] is [true]
#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    /// Determines if this Let is mutable
    pub mutable: bool,

    /// Let identifier
    pub id: Id,

    /// Expression which determines initial Let state
    pub expr: Box<Expr>,
}

impl From<Let> for ExprKind {
    fn from(kind: Let) -> ExprKind {
        ExprKind::Let(kind)
    }
}

/// Let setter for overwriting data in an existing [Let] whilst
/// [Let::mutable] is [true]
#[derive(Debug, Clone, PartialEq)]
pub struct SetLet {
    /// Let identifier ([Id::range.start] should be used as the start)
    pub id: Id,

    /// Expression determining what [SetLet::id] should be set to
    pub expr: Box<Expr>,
}

impl From<SetLet> for ExprKind {
    fn from(kind: SetLet) -> ExprKind {
        ExprKind::SetLet(kind)
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
pub struct StrLit(pub String);

impl From<StrLit> for ExprKind {
    fn from(kind: StrLit) -> ExprKind {
        ExprKind::StrLit(kind)
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

/// Bool literal used for defining raw bools
#[derive(Debug, Clone, PartialEq)]
pub struct BoolLit(pub bool);

impl From<BoolLit> for ExprKind {
    fn from(kind: BoolLit) -> ExprKind {
        ExprKind::BoolLit(kind)
    }
}
