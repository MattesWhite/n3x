//! Abstract syntax representation.

use super::Rule;
use crate::traverse::AbstractNode;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AbstractSyntaxNode<'i> {
    txt: &'i str,
    sup: Rule,
    sub: Rule,
    children: Vec<AbstractSyntaxNode<'i>>,
}

impl<'i> AbstractNode<'i, Rule> for AbstractSyntaxNode<'i> {
    fn from_rule_and_str(rule: Rule, txt: &'i str) -> Self {
        Self {
            txt,
            sup: rule,
            sub: rule,
            children: vec![],
        }
    }
    fn children(&self) -> &[Self] {
        self.children.as_slice()
    }
    fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }
    fn rule(&self) -> Rule {
        self.sup
    }
    fn rule_mut(&mut self) -> &mut Rule {
        &mut self.sub
    }
    fn as_str(&self) -> &'i str {
        self.txt
    }
}

impl<'i> fmt::Display for AbstractSyntaxNode<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

impl<'i> AbstractSyntaxNode<'i> {
    pub fn super_rule(&self) -> Rule {
        self.sup
    }
    pub fn sub_rule(&self) -> Rule {
        self.sub
    }
    fn write_indented<W: fmt::Write>(&self, w: &mut W, level: usize) -> fmt::Result {
        indent(w, level)?;
        self.write_rule(w)?;
        if self.txt.len() > 60 {
            writeln!(w, "{}...", &self.txt[..60])?;
        } else {
            writeln!(w, "{}", self.txt)?;
        }
        let nxt = level + 1;
        for chd in self.children.iter() {
            chd.write_indented(w, nxt)?;
        }
        Ok(())
    }
    pub fn write_rule<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{:?}", self.sup)?;
        if self.sub != self.sup {
            write!(w, " ({:?})", self.sub)?;
        }
        write!(w, ": ")
    }
}

fn indent<W: fmt::Write>(w: &mut W, level: usize) -> fmt::Result {
    for _ in 0..level {
        write!(w, "  ")?;
    }
    Ok(())
}
