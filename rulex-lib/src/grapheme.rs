//! Contains the [`Grapheme`] type, which matches a
//! [Unicode grapheme](https://www.regular-expressions.info/unicode.html#grapheme).

use crate::{
    compile::{Compile, CompileResult, CompileState},
    error::{CompileErrorKind, Feature},
    options::{CompileOptions, RegexFlavor},
    span::Span,
};

/// The `X` expression, matching a
/// [Unicode grapheme](https://www.regular-expressions.info/unicode.html#grapheme).
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg", derive(Debug))]
pub struct Grapheme {
    pub(crate) span: Span,
}

impl Compile for Grapheme {
    fn comp(
        &self,
        options: CompileOptions,
        _state: &mut CompileState,
        buf: &mut String,
    ) -> CompileResult {
        if options.flavor == RegexFlavor::JavaScript {
            return Err(
                CompileErrorKind::Unsupported(Feature::Grapheme, options.flavor).at(self.span),
            );
        }
        buf.push_str("\\X");
        Ok(())
    }
}
