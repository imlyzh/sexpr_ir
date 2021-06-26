use std::vec;
use std::{fs::File, io::Read, path::Path};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::gast::constant::Constant;
use crate::gast::list::List;
use crate::gast::symbol::{Localation, Symbol};
use crate::{error::CompilerError, utils::escape_str};
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
        debug_assert_eq!(pair.as_rule(), Rule::sexpr);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::list => Self::List(Handle::new(List::parse_from(pair))),
            Rule::constant => Self::Const(Constant::parse_from(pair)),
            Rule::quote | Rule::unquote => {
                let (line, colum) = pair.as_span().start_pos().line_col();
                let pos = pair.as_span().start_pos().pos();
                let pos = Localation::new(line, colum, pos);

                let quote = if pair.as_rule() == Rule::quote {
                    "quote"
                } else {
                    "unquote"
                };
                let quote = Symbol::from(quote, &pos);
                let quote = GAst::Const(Constant::Sym(Handle::new(quote)));

                let value = GAst::parse_from(pair.into_inner().next().unwrap());

                let lst = List(vec![quote, value], None);
                Self::List(Handle::new(lst))
            }
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for Constant {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::constant);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::symbol => Self::Sym(Handle::new(Symbol::parse_from(pair))),
            Rule::string_lit => Self::Str(Handle::new(escape_str(pair.as_str()))),
            Rule::uint_lit => Self::Uint(pair.as_str().parse().unwrap()),
            Rule::int_lit => Self::Int(pair.as_str().parse().unwrap()),
            Rule::float_lit => Self::Float(pair.as_str().parse().unwrap()),
            Rule::bool_lit => Self::Bool(pair.as_str().parse().unwrap()),
            // Rule::char_lit => Self::Char(str2char(&escape_str(pair.as_str()))),
            Rule::nil_lit => Self::Nil,
            _ => unreachable!()
        }
    }
}

impl ParseFrom<Rule> for List {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::list);
        let pair = pair.into_inner().next().unwrap();
        debug_assert_eq!(pair.as_rule(), Rule::list_core);
        let r: Vec<_> = pair.into_inner().collect();
        match r.len() {
            1 => List(vec![GAst::parse_from(r.get(0).unwrap().clone())], None),
            x if x > 1 => {
                let mut list: Vec<_> = r[..r.len()-1].iter()
                .cloned()
                .map(GAst::parse_from)
                .collect();
                let pair_right = r.last().unwrap();
                let pair_right =
                    if pair_right.as_rule() == Rule::pair_right {
                        let r = pair_right.clone().into_inner().next().unwrap();
                        debug_assert_eq!(r.as_rule(), Rule::sexpr);
                        Some(GAst::parse_from(r))
                    } else {
                        list.push(GAst::parse_from(pair_right.clone()));
                        None
                    };
                List(list, pair_right)
            }
            _ => List(vec![], None)
        }
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
    Ok(List(result, None))
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
        .next().unwrap()
        .into_inner()
        .next().unwrap();
    Ok(GAst::parse_from(pair))
}
