pub mod constant;
pub mod list;
pub mod symbol;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::Arc,
};

use self::{constant::Constant, list::List, symbol::Symbol};

pub type Handle<T> = Arc<T>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GAst {
    Const(Constant),
    List(Handle<List>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record(pub HashMap<Handle<Symbol>, GAst>);

impl Display for GAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_gast_case!(self, f, Const);
        fmt_gast_case!(self, f, List);
        unreachable!()
    }
}

impl GAst {
    impl_is_type!(is_const, Const);
    impl_is_type!(is_list, List);
}

impl GAst {
    impl_get_item!(get_const, Const, Constant);
    impl_get_item!(get_list, List, Handle<List>);
}
