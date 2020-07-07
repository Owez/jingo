//! AST (abstract-syntax-tree) for Jingo, containing the core concepts of the
//! infomation structure regarding the compilers internals.
//!
//! If you want to create this AST, see [crate::frontend::parser].

/// A statement, typically a general flow change in the language being a
/// logic/data barrier that needs to be delt with.
pub enum StatementNode {
    Fun(Fun),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
    Variable(Variable),
    Class(Class),
    If(If),
    /// A simple print statement, similar in style as a [Return] statement but prints
    /// the expression given.
    Print(ExpressionNode),
    /// An expression node contained within a statement, typically similar to
    /// `1 + 1;` or a method on a class.
    Expression(ExpressionNode),
    /// Return line for a [Fun].
    Return(ExpressionNode),
}

/// An expression, something in code that can be calculated to determine it's
/// value.    
pub enum ExpressionNode {
    /// A binary operation recursing down into two further expressions.
    BinOp(Box<ExpressionNode>, BinOp, Box<ExpressionNode>),
    Constant(Constant),
    FunCall(FunCall),
}

/// A simple if statement.
///
/// # Examples
///
/// ```jingo
/// if 1 + 2 == 3 {
///     print "Basic logic works!";
/// }
/// ```
pub struct If {
    /// Condition for if to be executed
    pub condition: ExpressionNode,
    /// Body inside if statement
    pub body: Vec<StatementNode>,
}

/// A single binary operation token, stemming directly from the lexer's token
/// types for this situation.
pub enum BinOp {
    /// Addition, `+`
    Add,
    /// Subtraction, `-`
    Sub,
    /// Division, `/`
    Div,
    /// Multiplication, `*`
    Mul,
    /// Power of, `^`
    Power,
    /// Modulo, `%`
    Mod,
    /// Equal to, `==`
    EqualTo,
    /// Not equal to, `=!`
    NotEqualTo,
    /// Greater than, `>`
    GreaterThan,
    /// Greater than or equal, `>=`
    GreaterThanOrEqual,
    /// Less than, `<`
    LessThan,
    /// Less than or equal, `<=`
    LessThanOrEqual,
    /// And, `and`
    And,
    /// Or, `or`
    Or,
}

/// A function call to enact a function.
///
/// # Examples
///
/// ```zypo
/// fun my_function(hi) {
///     print hi; # will be printed once called
/// }
///
/// my_function("nice"); # call function with params
/// ```
pub struct FunCall {
    /// Identifier given, e.g. `my_func` of `my_func(1, 2 + 3)`
    pub ident: String,

    /// The parameters given, not length checked compared to the actual func(s)
    pub params: Vec<ExpressionNode>,
}

/// A literal that can be directly used inside of the finished binary without
/// any further computation.
pub enum Constant {
    /// Integer, e.g. `34` of `var i = 34;`
    Int(i32),
    /// String, e.g. contents inside `""` of `var x = "contents";`
    Str(String),
    /// Boolean, e.g. `true` and `false` of `var y = true and false;`
    Bool(bool),
}

/// A function signature and body.
///
/// # Examples
///
/// ```jingo
/// fun my_function(parameter, other_param) {
///     -- this is the body of the function
///
///     return parameter * other_param; -- multiply and return
/// }
/// ```
pub struct Fun {
    /// Identifier, e.g. `my_function` of `fun my_function(x) {}`
    pub ident: String,

    /// Given parameters to use, as this is a dynamic language they are
    /// essentially a collection of identifiers to use at a later date
    pub parmas: Vec<String>,

    /// A markdown-compatible [String] that is a documenation comment.
    ///
    /// NOTE: This may be bound to a trait in the future to extend to other
    /// datatypes.
    pub docs: Option<String>,
}

/// A variable inside of Zypo, the most common datastructure that can change.
pub struct Variable {
    /// Identifier, e.g. `x` of `var x = 0;`
    pub ident: String,

    /// The body of the variable to be evaluated at a later date
    pub body: Box<ExpressionNode>,
}
