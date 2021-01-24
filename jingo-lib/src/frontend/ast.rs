//! Expression-centric abstract syntax tree for Jingo

use crate::meta::MetaPos;

use std::fmt;

/// Expression kind enumeration for the AST, containing all possible varients for
/// the AST to use
///
/// You may be wanting to see [Expr], which is this main enumeration, combined
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
pub enum Expr {
    BinOp(BinOp),
    Class(Class),
    Function(Function),
    Method(Method),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BinOp(_) => write!(f, ""),
            Expr::Class(class) => write!(f, "{}", class),
            Expr::Function(function) => write!(f, "{}", function),
            Expr::Method(method) => write!(f, "{}", method),
        }
    }
}

/// Binary operation varients, defining allowed types of a [BinOp] expression
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

/// Binary operation allowing two [Expr]s to be modified by a mathmatical notation
pub struct BinOp {
    /// Leftmost expression
    pub left: Box<Expr>,

    /// Rightmost expression
    pub right: Box<Expr>,

    /// Mathmatical notation to modifiy [BinOp::left] and [BinOp::right] together by
    pub kind: BinOpKind,

    /// Positional data for [BinOp::kind], should typically be 1 behind [BinOp::right]
    pub kind_pos: MetaPos,
}

/// Documentation string, commonly refered to as a "docstring", used to document
/// [Expr] variants for documentation generation
pub struct Doc {
    /// Actual documentation infomation added by programmer
    pub inner: String,

    /// Positional data
    pub pos: MetaPos,
}

impl fmt::Display for Doc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Pre-validated valid identifier
pub struct Id {
    /// Actual identifier name/data programmer passed
    pub inner: String,

    /// Positional data
    pub pos: MetaPos,
}

/// Class definition
pub struct Class {
    /// Class documentation
    pub doc: Option<Doc>,

    /// Name of class
    pub name: Id,

    /// Start position of this class
    pub pos: MetaPos,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.doc {
            Some(doc) => write!(f, "{}", doc),
            None => write!(f, ""),
        }
    }
}

/// Subprogram allowing code modularity, recurses down into more [Expr]
/// nodes. This is different from the [Method] structure as this one is for
/// non-class-linked subprograms
pub struct Function {
    /// Function documentation
    pub doc: Option<Doc>,

    /// Allowed arguments to be passed
    pub args: Vec<String>,

    /// Body of function
    pub body: Vec<Expr>,

    /// Start position of this function
    pub pos: MetaPos,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.doc {
            Some(doc) => write!(f, "{}", doc),
            None => write!(f, ""),
        }
    }
}

/// Class-linked subprogram similar to the base [Function], but is strictly linked
/// to a certain class
pub struct Method {
    /// Method documentation
    pub doc: Option<Doc>,

    /// Allowed arguments to be passed
    pub args: Vec<Id>,

    /// Body of method
    pub body: Vec<Expr>,

    /// Reference to the class name (which should be an existing [Class]) this
    /// method is linked to
    pub class_name: Id,

    /// Distinguishes between a creation method (defined with `::`) or a normal
    /// method (defined with `.`)
    pub creation_method: bool,

    /// Start position of this method
    pub pos: MetaPos,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.doc {
            Some(doc) => write!(f, "{}", doc),
            None => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_doc() {
        assert_eq!(
            format!(
                "{}",
                Expr::Class(Class {
                    name: Id {
                        inner: "SomeClass".to_string(),
                        pos: MetaPos::new()
                    },
                    doc: Some(Doc {
                        inner: "hi".to_string(),
                        pos: MetaPos::new()
                    }),
                    pos: MetaPos::new()
                })
            ),
            "hi".to_string()
        )
    }

    #[test]
    fn function_doc() {
        assert_eq!(
            format!(
                "{}",
                Expr::Function(Function {
                    doc: Some(Doc {
                        inner: "hi".to_string(),
                        pos: MetaPos::new()
                    }),
                    args: vec![],
                    body: vec![],
                    pos: MetaPos::new()
                })
            ),
            "hi".to_string()
        )
    }

    #[test]
    fn method_doc() {
        assert_eq!(
            format!(
                "{}",
                Expr::Method(Method {
                    doc: Some(Doc {
                        inner: "hi".to_string(),
                        pos: MetaPos::new()
                    }),
                    args: vec![],
                    body: vec![],
                    class_name: Id {
                        inner: "Hi".to_string(),
                        pos: MetaPos::new()
                    },
                    creation_method: false,
                    pos: MetaPos::new()
                })
            ),
            "hi".to_string()
        )
    }
}
