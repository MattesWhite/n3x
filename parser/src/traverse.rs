//! Functions to traverse a syntax tree.

use pest::{
    iterators::{Pair, Pairs},
    RuleType,
};
use std::fmt;

pub trait AbstractNode<'i, R: RuleType>: Sized + fmt::Debug {
    fn from_pair(p: &Pair<'i, R>) -> Self {
        Self::from_rule_and_str(p.as_rule(), p.as_str().trim())
    }
    fn from_rule_and_str(rule: R, txt: &'i str) -> Self;
    fn children(&self) -> &[Self];
    fn add_child(&mut self, child: Self);
    fn rule(&self) -> R;
    fn rule_mut(&mut self) -> &mut R;
    fn as_str(&self) -> &'i str;
}

pub fn ast_from_pair<'i, R, AN>(root: Pair<'i, R>) -> AN
where
    R: RuleType,
    AN: AbstractNode<'i, R>,
{
    let mut ast = AN::from_pair(&root);
    expand_root(&mut ast, root.into_inner());
    ast
}

fn expand_root<'i, R, AN>(root: &mut AN, ps: Pairs<'i, R>)
where
    R: RuleType,
    AN: AbstractNode<'i, R>,
{
    for p in ps {
        let txt = p.as_str().trim();
        if txt != root.as_str() {
            let mut node = AN::from_rule_and_str(p.as_rule(), txt);
            expand_root(&mut node, p.into_inner());
            root.add_child(node);
        } else {
            *(root.rule_mut()) = p.as_rule();
            expand_root(root, p.into_inner());
        }
    }
}

pub fn print_flattened<'i, R: RuleType>(parent: Option<&'i str>, this: Pair<'i, R>, level: usize) {
    let mut level = level;
    if let Some(parent) = parent {
        if parent.trim() != this.as_str().trim() {
            indent(level);
            print_pair(&this);
            level += 1;
        } else {
            print!(" -> {}", shortened_rule(&this));
        }
        print_inner(this, level);
    } else {
        // root
        print_pair(&this);
        level += 1;
        print_inner(this, level);
    }
}

fn indent(level: usize) {
    println!("");
    for _ in 0..level {
        print!("  ");
    }
}

fn print_pair<'i, R: RuleType>(p: &Pair<'i, R>) {
    print!("{:?}: ", p.as_rule());
    let txt = p.as_str();
    if txt.len() > 80 {
        print!("{}...", &txt[..80]);
    } else {
        print!("{}", txt);
    }
}

fn print_inner<'i, R: RuleType>(this: Pair<'i, R>, level: usize) {
    let txt = this.as_str();
    for p in this.into_inner() {
        print_flattened(Some(txt), p, level);
    }
}

fn shortened_rule<'i, R: RuleType>(this: &Pair<'i, R>) -> String {
    let rule = format!("{:?}", this.as_rule());
    if rule.len() > 10 {
        format!("{}...", &rule[..8])
    } else {
        rule
    }
}
