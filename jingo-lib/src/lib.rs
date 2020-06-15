//! # Jingo Library
//!
//! The central library for Jingo, containing the core of the compiler.
//!
//! This library is designed to be used downstream for the official CLI or any
//! future language servers/other tooling utilising the compiler without wanting
//! the added bulk of CLI dependencies.
//!
//! All frontend (e.g. lexing, parsing, ast) are contained in the [crate::frontend]
//! module and all backend parts (e.g. codegen) are contained in [crate::backend]
//! if you need to interact with a specific part of the compiler.
//!
//! ## Usage
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! jingo-lib = "0.1"
//! ```

pub mod backend;
pub mod frontend;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
