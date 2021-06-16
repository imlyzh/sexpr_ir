use std::{fs::File, io::Read, path::Path};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::gast::list::List;
use crate::gast::symbol::{Localation, Symbol};
use crate::{error::CompilerError, utils::{escape_str, str2char}};
use crate::gast::*;

#[derive(Parser)]
#[grammar = "./syntax/sexpr/grammar.pest"]
pub struct Cement {}

pub type ParseError = Error<Rule>;

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

impl ParseFrom<Rule> for GAst {
    fn parse_from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::list => GAst::List(Handle::new(List::parse_from(pair))),
            Rule::pair => GAst::Pair(Handle::new(pair::Pair::parse_from(pair))),
            Rule::symbol => GAst::Sym(Handle::new(Symbol::parse_from(pair))),
            Rule::string_lit => GAst::Str(Handle::new(escape_str(pair.as_str()))),
            Rule::uint_lit => GAst::Uint(pair.as_str().parse().unwrap()),
            Rule::int_lit => GAst::Int(pair.as_str().parse().unwrap()),
            Rule::float_lit => GAst::Float(pair.as_str().parse().unwrap()),
            Rule::bool_lit => GAst::Bool(pair.as_str().parse().unwrap()),
            Rule::char_lit => GAst::Char(str2char(&escape_str(pair.as_str()))),
            Rule::nil_lit => GAst::Nil,
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for List {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::list);
        let r = pair.into_inner()
            .flat_map(|x| x.into_inner())
            .map(GAst::parse_from)
            .collect();
        List(r)
    }
}

impl ParseFrom<Rule> for pair::Pair {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::pair);
        let mut iter = pair.into_inner();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        pair::Pair(GAst::parse_from(a), GAst::parse_from(b))
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol);
        let (line, colum) = pair.as_span().start_pos().line_col();
        let pos = pair.as_span().start_pos().pos();
        let pos = Localation::new(line, colum, pos);
        Symbol::from(pair.as_str(), &pos)
    }
}

pub fn parse_unit(pair: Pair<Rule>) -> Option<GAst> {
    debug_assert_eq!(pair.as_rule(), Rule::unit);
    match pair.as_rule() {
        Rule::sexpr => Some(GAst::parse_from(pair.clone().into_inner().next().unwrap())),
        Rule::EOI => None,
        _ => unreachable!(),
    }
}

pub fn parse(input: &str) -> Result<List, ParseError> {
    let pairs: Pairs<Rule> = Cement::parse(Rule::unit, input)?;
    let result = pairs
        .flat_map(|x| x.into_inner())
        .filter_map(parse_unit)
        .collect();
    Ok(List(result))
}

pub fn file_parse(path: &Path) -> Result<List, CompilerError<ParseError>> {
	let file_path = Handle::new(Symbol::new(path.to_str().unwrap()));
    let mut f = if let Ok(f) = File::open(path) {
        f
    } else {
        return Err(CompilerError::FileOpenError(file_path));
    };
	let mut buf = String::new();
	f.read_to_string(&mut buf).or(Err(CompilerError::FileOpenError(file_path)))?;
	parse(&buf).map_err(CompilerError::ParseError)
}

pub fn repl_parse(input: &str) -> Result<GAst, ParseError> {
    let pair = Cement::parse(Rule::repl_unit, input)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();
    Ok(GAst::parse_from(pair))
}
