mod common;
mod decl;
mod expr;
mod stmt;

use std::io::Write;

use pest::Parser as PestParser;

use crate::ast;
use crate::common::Generator;

pub use self::common::Error;
use self::common::{grammar_error_static, ParseResult, Rule, TeaLangParser};

pub struct Parser<'a> {
    input: &'a str,
    pub program: Option<Box<ast::Program>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            program: None,
        }
    }
}

impl<'a> Generator for Parser<'a> {
    type Error = Error;

    fn generate(&mut self) -> Result<(), Error> {
        let ctx = ParseContext::new(self.input);
        self.program = Some(ctx.parse()?);
        Ok(())
    }

    fn output<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        let ast = self
            .program
            .as_ref()
            .ok_or_else(|| grammar_error_static("output before generate"))?;
        write!(w, "{ast}")?;
        Ok(())
    }
}

pub(crate) struct ParseContext<'a> {
    #[allow(dead_code)]
    input: &'a str,
}

impl<'a> ParseContext<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn parse(&self) -> ParseResult<Box<ast::Program>> {
        let pairs = <TeaLangParser as PestParser<Rule>>::parse(Rule::program, self.input)
            .map_err(|e| Error::Syntax(e.to_string()))?;

        let mut use_stmts = Vec::new();
        let mut elements = Vec::new();

        for pair in pairs {
            if pair.as_rule() == Rule::program {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::use_stmt => {
                            use_stmts.push(self.parse_use_stmt(inner)?);
                        }
                        Rule::program_element => {
                            if let Some(elem) = self.parse_program_element(inner)? {
                                elements.push(*elem);
                            }
                        }
                        Rule::EOI => {}
                        _ => {}
                    }
                }
            }
        }

        Ok(Box::new(ast::Program {
            use_stmts,
            elements,
        }))
    }
}
