pub mod constant;
pub mod symbol;
pub mod list;
pub mod pair;

use std::{collections::HashMap, fmt::{Debug, Display}, sync::Arc};
use serde::{Serialize, Deserialize};

use self::{constant::Constant, list::List, pair::Pair, symbol::Symbol};

pub type Handle<T> = Arc<T>;


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum GAst {
    Const(Constant),
    Sym(Handle<Symbol>),
    List(Handle<List>),
    Pair(Handle<Pair>),
}


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Record (pub HashMap<Handle<Symbol>, GAst>);


impl GAst {
    impl_is_type!(is_const, Const);
    impl_is_type!(is_sym, Sym);
    impl_is_type!(is_pair, Pair);
    impl_is_type!(is_list, List);
}

impl GAst {
    impl_get_item!(get_const, Const, Constant);
    impl_get_item!(get_sym, Sym, Handle<Symbol>);
    impl_get_item!(get_pair, Pair, Handle<Pair>);
    impl_get_item!(get_list, List, Handle<List>);
}

impl Display for GAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_gast_case!(self, f, Const);
        fmt_gast_case!(self, f, Sym);
        fmt_gast_case!(self, f, Pair);
        fmt_gast_case!(self, f, List);
        unreachable!()
    }
}
