//! ExpressionInner-centric abstract syntax tree for Jingo

use crate::meta::MetaPos;

use std::fmt;

/// ExpressionInner enumeration for the AST, containing all possible varients for
/// the AST to use
///
/// You may be wanting to see [Expression], which is this main enumeration, combined
/// with the [MetaPos] structure to give context to the positioning of the node
/// in question
///
/// # Documentation generation
///
/// All varients included in this enumeration are covered under the [fmt::Display]
/// trait implementation included, allowing easy documentation generation once the
/// AST has been created from parsing.
///
/// Instances which do not have any user-given documentation (or dont allow entry
/// of such) will simply provide an empty string.
///
/// # Varient documentation
///
/// Any varients included in this enumeration which are not documentation mean
/// that documentation is provided in the item they are referencing. Take
/// [ExpressionInner::Class] --> [Class] as an example of this.
pub enum ExpressionInner {
    /// Binary operation allowing two [ExpressionInner]s to be modified by a mathmatical
    /// notation defined in [BinOp]
    BinOp((Box<ExpressionInner>, BinOp, Box<ExpressionInner>)),
    Class(Class),
    Function(Function),
    Method(Method),
}

impl fmt::Display for ExpressionInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionInner::BinOp(_) => write!(f, ""),
            ExpressionInner::Class(class) => write!(f, "{}", class.doc),
            ExpressionInner::Function(function) => write!(f, "{}", function.doc),
            ExpressionInner::Method(method) => write!(f, "{}", method.doc),
        }
    }
}

/// The most abstract definition for the AST, a fully-encompassed expression which
/// wraps [ExpressionInner] and [MetaPos] to give context
///
/// To get documentation infomation on this expression, you may use both the
/// [fmt::Display] on [Expression::inner] for the user-generated documentation or
/// get the positional data using the same trait implementation with [Expression::pos]
pub struct Expression {
    /// Type + data of this expression
    pub inner: ExpressionInner,

    /// Positional data for where this expression occurs
    pub pos: MetaPos,
}

impl From<Expression> for ExpressionInner {
    fn from(expr: Expression) -> Self {
        expr.inner
    }
}

impl From<Expression> for MetaPos {
    fn from(expr: Expression) -> Self {
        expr.pos
    }
}

/// Binary operation enumeration, defining allowed types of an [ExpressionInner::BinOp]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Documentation string, commonly refered to as a "docstring", used to document
/// [ExpressionInner] varients for documentation generation
pub struct Doc(Option<String>);

impl fmt::Display for Doc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(doc) => write!(f, "{}", doc),
            None => write!(f, ""),
        }
    }
}

/// Class definition
pub struct Class {
    /// Class documentation
    pub doc: Doc,
}

/// Subprogram allowing code modularity, recurses down into more [Expression]
/// nodes. This is different from the [Method] structure as this one is for
/// non-class-linked subprograms
pub struct Function {
    /// Function documentation
    pub doc: Doc,

    /// Allowed arguments to be passed
    pub args: Vec<String>,

    /// Body of function
    pub body: Vec<Expression>,
}

/// Class-linked subprogram similar to the base [Function], but is strictly linked
/// to a certain class
pub struct Method {
    /// Method documentation
    pub doc: Doc,

    /// Allowed arguments to be passed
    pub args: Vec<String>,

    /// Body of method
    pub body: Vec<Expression>,

    /// Reference to the class name (which should be an existing [Class]) this
    /// method is linked to
    pub class_name: String,

    /// Distinguishes between a creation method (defined with `::`) or a normal
    /// method (defined with `.`)
    pub creation_method: bool,
}
