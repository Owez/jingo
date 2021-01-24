//! Expression-centric abstract syntax tree for Jingo

use std::fmt;

/// Expression enumeration for the AST, the most abstract node type all other
/// AST items conform to, as everything is considered an expression
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
/// [Expression::Class] --> [Class] as an example of this.
pub enum Expression {
    /// Binary operation allowing two [Expression]s to be modified by a mathmatical
    /// notation defined in [BinOp]
    BinOp((Box<Expression>, BinOp, Box<Expression>)),
    Class(Class),
    Function(Function),
    Method(Method),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::BinOp(_) => write!(f, ""),
            Expression::Class(class) => write!(f, "{}", class.doc),
            Expression::Function(function) => write!(f, "{}", function.doc),
            Expression::Method(method) => write!(f, "{}", method.doc),
        }
    }
}

/// Binary operation enumeration, defining allowed types of an [Expression::BinOp]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Documentation string, commonly refered to as a "docstring", used to document
/// [Expression] varients for documentation generation
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

/// Class-linked subprogram similar to the base [Function], but is strictly linked to a certain class
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
