//! Very simple representation of N3X terms.
//! No inner logic applied.

use super::{AbstractSyntaxNode, Rule};
use crate::traverse::AbstractNode as _;
use anyhow::{anyhow, Result};
use std::borrow::Cow;

pub type Text<'a> = Cow<'a, str>;

pub enum Term<'a> {
    /// { statement . statement . ... }
    Formula(Vec<Statement<'a>>),
    /// ( term term term ... )
    Collection(Vec<Term<'a>>),
    /// IRI or literal
    Constant(Text<'a>),
    /// ?text
    Universal(Text<'a>),
    /// _:text
    Existential(Text<'a>),
    /// name(arg, arg, ...)
    /// name := built-in | IRI
    FunctionCall { name: Text<'a>, args: Vec<Term<'a>> },
}

impl<'i> Term<'i> {
    pub fn from_ast(ast: &AbstractSyntaxNode<'i>) -> Result<Self> {
        match (ast.super_rule(), ast.sub_rule()) {
            (Rule::document, _) => Term::from_ast(&ast.children()[0]), // Transform the inner formula
            (Rule::formula, _) => {
                let formula = ast
                    .children()
                    .iter()
                    .map(|node| Statement::from_ast(node))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Term::Formula(formula))
            }
            (_, Rule::literal)
            | (_, Rule::NumericLiteral)
            | (_, Rule::RDFLiteral)
            | (_, Rule::BooleanLiteral)
            | (_, Rule::iri)
            | (_, Rule::IRIREF)
            | (_, Rule::PrefixedName) => Ok(Term::Constant(ast.as_str().into())),
            (Rule::predicate, _)
                if ast.as_str() == "a"
                    || ast.as_str() == "=>"
                    || ast.as_str() == "<="
                    || ast.as_str() == "=" =>
            {
                Ok(Term::Constant(ast.as_str().into()))
            }
            (_, _) => {
                let mut rule = String::new();
                ast.write_rule(&mut rule)?;
                Err(anyhow!("Rule {} can not be converted into a N3X term", rule).into())
            }
        }
    }
}

pub enum Statement<'a> {
    /// @prefix ...
    DeclPrefix { prefix: Text<'a>, iri: Text<'a> },
    /// @base ...
    DeclBase(Text<'a>),
    /// s p o .
    Simple([Term<'a>; 3]),
    /// ?var := expr
    Bind { var: Text<'a>, expr: Term<'a> },
    /// filter += expr
    Filter(Term<'a>),
}

impl<'i> Statement<'i> {
    pub fn from_ast(ast: &AbstractSyntaxNode<'i>) -> Result<Self> {
        match (ast.super_rule(), ast.sub_rule()) {
            (Rule::statement, Rule::prefixID) => {
                let kids = ast.children();
                Ok(Statement::DeclPrefix {
                    prefix: kids[0].as_str().into(),
                    iri: kids[1].as_str().into(),
                })
            }
            (Rule::statement, Rule::base) => {
                Ok(Statement::DeclBase(ast.children()[0].as_str().into()))
            }
            (Rule::statement, Rule::simple_statement) => todo!("Simple | Bind | Filter"),
            (_, _) => {
                let mut rule = String::new();
                ast.write_rule(&mut rule)?;
                Err(anyhow!("Rule {} can not be converted into a N3X statement", rule).into())
            }
        }
    }
}
