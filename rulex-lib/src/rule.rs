use crate::{
    alternation::Alternation,
    boundary::Boundary,
    char_class::CharClass,
    compile::{Compile, CompileState},
    error::{CompileError, ParseError},
    group::Group,
    options::{CompileOptions, ParseOptions},
    repetition::Repetition,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Rulex<'i> {
    Literal(&'i str),
    CharClass(CharClass<'i>),
    Group(Group<'i>),
    Alternation(Alternation<'i>),
    Repetition(Box<Repetition<'i>>),
    Boundary(Boundary),
}

impl<'i> Rulex<'i> {
    pub fn parse(input: &'i str, _options: ParseOptions) -> Result<Self, ParseError> {
        crate::parse::parse(input)
    }

    pub fn compile(&self, options: CompileOptions) -> Result<String, CompileError> {
        let mut buf = String::new();
        self.comp(options, &mut CompileState::new(), &mut buf)?;
        Ok(buf)
    }

    pub fn parse_and_compile(input: &str, options: CompileOptions) -> Result<String, CompileError> {
        let parsed = Rulex::parse(input, options.parse_options)?;
        let mut buf = String::new();
        parsed.comp(options, &mut CompileState::new(), &mut buf)?;
        Ok(buf)
    }

    pub fn negate(self) -> Option<Self> {
        match self {
            Rulex::CharClass(c) => Some(Rulex::CharClass(c.negate())),
            Rulex::Boundary(b) => Some(Rulex::Boundary(match b {
                Boundary::Word => Boundary::NotWord,
                Boundary::NotWord => Boundary::Word,
                Boundary::Start | Boundary::End => return None,
            })),
            _ => None,
        }
    }

    pub(crate) fn needs_parens_before_repetition(&self) -> bool {
        match self {
            Rulex::Literal(_) | Rulex::Alternation(_) => true,
            Rulex::Group(g) => g.needs_parens_before_repetition(),
            Rulex::CharClass(_) => false,
            Rulex::Repetition(_) => false,
            Rulex::Boundary(_) => false,
        }
    }
}

#[cfg(feature = "dbg")]
impl core::fmt::Debug for Rulex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Literal(arg0) => arg0.fmt(f),
            Self::CharClass(arg0) => arg0.fmt(f),
            Self::Group(arg0) => arg0.fmt(f),
            Self::Alternation(arg0) => arg0.fmt(f),
            Self::Repetition(arg0) => arg0.fmt(f),
            Self::Boundary(arg0) => arg0.fmt(f),
        }
    }
}

impl Compile for Rulex<'_> {
    fn comp(
        &self,
        options: CompileOptions,
        state: &mut crate::compile::CompileState,
        buf: &mut String,
    ) -> crate::compile::CompileResult {
        match self {
            Rulex::Literal(l) => l.comp(options, state, buf),
            Rulex::CharClass(c) => c.comp(options, state, buf),
            Rulex::Group(g) => g.comp(options, state, buf),
            Rulex::Alternation(a) => a.comp(options, state, buf),
            Rulex::Repetition(r) => r.comp(options, state, buf),
            Rulex::Boundary(b) => b.comp(options, state, buf),
        }
    }
}
