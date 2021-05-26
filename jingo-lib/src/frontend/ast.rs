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
    Path(Path),
    Class(Class),
    Function(Function),
    Method(Method),
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    If(If),
    While(While),
    Return(Return),
    Let(Let),
    LetSet(LetSet),
    LetCall(LetCall),
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
    fn from(kind: Not) -> Self {
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
    fn from(kind: Op) -> Self {
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

/// Path to something, `::` seperated
///
/// # Internals
///
/// When parsing from a [Token::Path], it may be modified to remove the last few
/// elements for nodes like [FunctionCall] with it's [FunctionCall::id] element
#[derive(Debug, Clone, PartialEq)]
pub struct Path(pub Vec<Id>);

impl Path {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn last(&mut self) -> Option<Id> {
        self.0.pop()
    }

    pub fn last_2(&mut self) -> Option<(Id, Id)> {
        match self.0.pop() {
            Some(first) => match self.0.pop() {
                Some(second) => Some((first, second)),
                None => None,
            },
            None => None,
        }
    }

    pub fn local(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Path> for ExprKind {
    fn from(kind: Path) -> Self {
        ExprKind::Path(kind)
    }
}

/// Class definition
#[derive(Debug, Clone, PartialEq)]
pub struct Class(pub Id);

impl From<Class> for ExprKind {
    fn from(kind: Class) -> Self {
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
    fn from(kind: Function) -> Self {
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
    fn from(kind: Method) -> Self {
        ExprKind::Method(kind)
    }
}

/// Caller for a function, allows invoking functions with passed arguments
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// Identifier of the function
    pub id: Id,

    /// Path before [FunctionCall::Id] for scoping
    pub path: Path,

    /// Argument to pass and invoke within the function
    pub args: Vec<Expr>,
}

impl From<FunctionCall> for ExprKind {
    fn from(kind: FunctionCall) -> Self {
        ExprKind::FunctionCall(kind)
    }
}

/// Caller for a method, allows invoking methods with passed arguments
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCall {
    /// Reference to the class name (which should be an existing [Class]) the
    /// method is linked to
    pub class_id: Id,

    /// Identifier of the function
    pub id: Id,

    /// Path before [MethodCall::class_id] and [MethodCall::Id] for scoping
    pub path: Path,

    /// Argument to pass and invoke within the function
    pub args: Vec<Expr>,
}

impl From<MethodCall> for ExprKind {
    fn from(kind: MethodCall) -> Self {
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
    fn from(kind: If) -> Self {
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
    fn from(kind: While) -> Self {
        ExprKind::While(kind)
    }
}

/// Return expression allowing pass-back from functions
#[derive(Debug, Clone, PartialEq)]
pub struct Return(pub Box<Expr>);

impl From<Return> for ExprKind {
    fn from(kind: Return) -> Self {
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
    fn from(kind: Let) -> Self {
        ExprKind::Let(kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetCall {
    /// Let identifier
    pub id: Id,

    /// Path to identifier
    pub path: Path,
}

impl From<Id> for LetCall {
    fn from(id: Id) -> Self {
        Self {
            id,
            path: Path::new(),
        }
    }
}

impl From<LetCall> for ExprKind {
    fn from(kind: LetCall) -> Self {
        ExprKind::LetCall(kind)
    }
}

/// Let setter for overwriting data in an existing [Let] whilst
/// [Let::mutable] is [true]
#[derive(Debug, Clone, PartialEq)]
pub struct LetSet {
    /// Let identifier
    pub id: Id,

    /// Path to identifier
    pub path: Path,

    /// Expression determining what [LetSet::id] should be set to
    pub expr: Box<Expr>,
}

impl From<LetSet> for ExprKind {
    fn from(kind: LetSet) -> Self {
        ExprKind::LetSet(kind)
    }
}

/// Integer literal used for defining raw integers
#[derive(Debug, Clone, PartialEq)]
pub struct IntLit(pub i64);

impl From<IntLit> for ExprKind {
    fn from(kind: IntLit) -> Self {
        ExprKind::IntLit(kind)
    }
}

/// Float literal used for defining raw floats
#[derive(Debug, Clone, PartialEq)]
pub struct FloatLit(pub f64);

impl From<FloatLit> for ExprKind {
    fn from(kind: FloatLit) -> Self {
        ExprKind::FloatLit(kind)
    }
}

/// String literal used for defining raw strings
#[derive(Debug, Clone, PartialEq)]
pub struct StrLit(pub String);

impl From<StrLit> for ExprKind {
    fn from(kind: StrLit) -> Self {
        ExprKind::StrLit(kind)
    }
}

/// Char literal used for defining raw chars
#[derive(Debug, Clone, PartialEq)]
pub struct CharLit(pub char);

impl From<CharLit> for ExprKind {
    fn from(kind: CharLit) -> Self {
        ExprKind::CharLit(kind)
    }
}

/// Bool literal used for defining raw bools
#[derive(Debug, Clone, PartialEq)]
pub struct BoolLit(pub bool);

impl From<BoolLit> for ExprKind {
    fn from(kind: BoolLit) -> Self {
        ExprKind::BoolLit(kind)
    }
}
