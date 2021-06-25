pub mod constant;
pub mod symbol;
pub mod list;
pub mod pair;
pub mod types;

use std::{collections::HashMap, fmt::{Debug, Display}, sync::Arc};
use serde::{Serialize, Deserialize};

use self::{constant::Constant, list::List, pair::Pair, symbol::Symbol, types::{GAstType, GetSimpleType, GetType, SimpleType}};

pub type Handle<T> = Arc<T>;


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum GAst {
    Const(Constant),
    List(Handle<List>),
    Pair(Handle<Pair>),
}


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Record (pub HashMap<Handle<Symbol>, GAst>);


impl Display for GAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_gast_case!(self, f, Const);
        fmt_gast_case!(self, f, Pair);
        fmt_gast_case!(self, f, List);
        unreachable!()
    }
}

impl GAst {
    impl_is_type!(is_const, Const);
    impl_is_type!(is_pair, Pair);
    impl_is_type!(is_list, List);
}

impl GAst {
    impl_get_item!(get_const, Const, Constant);
    impl_get_item!(get_pair, Pair, Handle<Pair>);
    impl_get_item!(get_list, List, Handle<List>);
}

impl GetSimpleType for GAst {
    fn get_simple_type(&self) -> SimpleType {
        if let Self::Const(c) = self {
            return c.get_simple_type();
        }
        impl_get_simple_type!(self, List);
        impl_get_simple_type!(self, Pair);
        unreachable!()
    }
}

impl GetType for GAst {
    fn get_type(&self) -> GAstType {
        if let Self::Const(c) = self {
            return c.get_type();
        }
        if let Self::Pair(x) = self {
            return x.get_type();
        }
        if let Self::List(x) = self {
            return x.get_type();
        }
        unreachable!()
    }
}
