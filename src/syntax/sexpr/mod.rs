use std::vec;
use std::{fs::File, io::Read};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::gast::constant::Constant;
use crate::gast::list::List;
use crate::gast::symbol::{Location, Symbol};
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
    fn parse_from(pair: Pair<T>, path: Handle<String>) -> Self;
}

impl ParseFrom<Rule> for GAst {
    fn parse_from(pair: Pair<Rule>, path: Handle<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::sexpr);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::list => Self::List(Handle::new(List::parse_from(pair, path))),
            Rule::constant => Self::Const(Constant::parse_from(pair, path)),
            Rule::quote | Rule::unquote => {
                let (line, colum) = pair.as_span().start_pos().line_col();
                let pos = pair.as_span().start_pos().pos();
                let pos = Location::new(path.clone(), line, colum, pos);

                let quote = if pair.as_rule() == Rule::quote {
                    "quote"
                } else {
                    "unquote"
                };
                let quote = Symbol::from(quote, &pos);
                let quote = GAst::Const(Constant::Sym(Handle::new(quote)));

                let value = GAst::parse_from(pair.into_inner().next().unwrap(), path);

                let lst = List(vec![quote, value], None);
                Self::List(Handle::new(lst))
            }
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for Constant {
    fn parse_from(pair: Pair<Rule>, path: Handle<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::constant);
        let pair = if let Some(x) = pair.into_inner().next() {
            x
        } else {
            return Constant::Nil;
        };
        match pair.as_rule() {
            Rule::symbol => Self::Sym(Handle::new(Symbol::parse_from(pair, path))),
            Rule::string_lit => Self::Str(Handle::new(escape_str(&pair.as_str()[1..pair.as_str().len()-1]))),
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
    fn parse_from(pair: Pair<Rule>, path: Handle<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::list);
        let pair = pair.into_inner().next().unwrap();
        debug_assert_eq!(pair.as_rule(), Rule::list_core);
        let r: Vec<_> = pair.into_inner().collect();
        match r.len() {
            1 => List(vec![GAst::parse_from(r.get(0).unwrap().clone(), path)], None),
            x if x > 1 => {
                let mut list: Vec<_> = r[..r.len()-1].iter()
                .cloned()
                .map(|x|GAst::parse_from(x, path.clone()))
                .collect();
                let pair_right = r.last().unwrap();
                let pair_right =
                    if pair_right.as_rule() == Rule::pair_right {
                        let r = pair_right.clone().into_inner().next().unwrap();
                        debug_assert_eq!(r.as_rule(), Rule::sexpr);
                        Some(GAst::parse_from(r, path))
                    } else {
                        list.push(GAst::parse_from(pair_right.clone(), path));
                        None
                    };
                List(list, pair_right)
            }
            _ => List(vec![], None)
        }
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>, path: Handle<String>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol);
        let (line, colum) = pair.as_span().start_pos().line_col();
        let pos = pair.as_span().start_pos().pos();
        let pos = Location::new(path, line, colum, pos);
        Symbol::from(pair.as_str(), &pos)
    }
}

pub fn parse_unit(pair: Pair<Rule>, path: Handle<String>) -> Vec<GAst> {
    pair.into_inner().filter_map(|x| match x.as_rule() {
        Rule::sexpr => Some(GAst::parse_from(x, path.clone())),
        Rule::EOI => None,
        _ => unreachable!()
    }).collect()
}


pub fn parse(input: &str, path: Handle<String>) -> Result<Vec<GAst>, ParseError> {
    let pairs: Pairs<Rule> = Cement::parse(Rule::unit, input)?;
    let result = pairs
        .flat_map(|x| parse_unit(x, path.clone()))
        .collect();
    Ok(result)
}

pub fn file_parse(path: &str) -> Result<Vec<GAst>, CompilerError<ParseError>> {
    use std::path::PathBuf;
    let path_buf = PathBuf::from(path).canonicalize().unwrap();
	let file_path = Handle::new(Symbol::new(path_buf.to_str().unwrap()));
    let mut f = if let Ok(f) = File::open(path_buf) {
        f
    } else {
        return Err(CompilerError::FileOpenError(file_path.0.clone()));
    };
	let mut buf = String::new();
    if f.read_to_string(&mut buf).is_err() {
        return Err(CompilerError::FileOpenError(file_path.0.clone()));
    }
	parse(&buf, file_path.0.clone()).map_err(CompilerError::ParseError)
}

pub fn repl_parse(input: &str) -> Result<GAst, ParseError> {
    let file_path = Handle::new(Symbol::new("<stdin>"));
    let pair = Cement::parse(Rule::repl_unit, input)?
        .next().unwrap()
        .into_inner()
        .next().unwrap();
    Ok(GAst::parse_from(pair, file_path.0.clone()))
}
