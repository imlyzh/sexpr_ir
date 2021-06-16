pub mod symbol;
pub mod list;
pub mod pair;

use std::{collections::HashMap, fmt::{Debug, Display}, sync::Arc};


use self::{
    list::List,
    pair::Pair,
    symbol::Symbol
};


pub type Handle<T> = Arc<T>;


#[derive(Debug, Clone, PartialEq)]
pub enum GAst {
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(Handle<String>),
    Sym(Handle<Symbol>),
    List(Handle<List>),
    Pair(Handle<Pair>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record (pub HashMap<Handle<Symbol>, GAst>);

macro_rules! impl_is_type {
    ($name:ident, $item:ident) => {
        pub fn $name(&self) -> bool {
            if let GAst::$item(_) = self {
                true
            } else {
                false
            }
        }
    };
}

impl GAst {
    impl_is_type!(is_bool, Bool);
    impl_is_type!(is_char, Char);
    impl_is_type!(is_int, Int);
    impl_is_type!(is_uint, Uint);
    impl_is_type!(is_float, Float);
    impl_is_type!(is_str, Str);
    impl_is_type!(is_sym, Sym);
    impl_is_type!(is_pair, Pair);
    impl_is_type!(is_list, List);
    pub fn is_nil(&self) -> bool {
		matches!(self, GAst::Nil)
    }
}

macro_rules! impl_get_item {
    ($name:ident, $item:ident, $tp:path) => {
        pub fn $name(&self) -> Option<$tp> {
            if let GAst::$item(x) = self {
                Some(x.clone())
            } else {
                None
            }
        }
    };
}

impl GAst {
    impl_get_item!(get_bool, Bool, bool);
    impl_get_item!(get_char, Char, char);
    impl_get_item!(get_int, Int, i64);
    impl_get_item!(get_uint, Uint, u64);
    impl_get_item!(get_float, Float, f64);
    impl_get_item!(get_str, Str, Handle<String>);
    impl_get_item!(get_sym, Sym, Handle<Symbol>);
    impl_get_item!(get_pair, Pair, Handle<Pair>);
    impl_get_item!(get_list, List, Handle<List>);
    pub fn get_nil(&self) -> Option<()> {
        if self.is_nil() { Some(()) } else { None }
    }
}


macro_rules! fmt_gast_case {
    ($self:ident, $f:ident, $item:ident) => {
        if let GAst::$item(x) = $self {
            return $f.write_str(&x.to_string());
        }
    };
}

impl Display for GAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let GAst::Nil = self {
            return f.write_str("nil");
        }
        fmt_gast_case!(self, f, Bool);
        fmt_gast_case!(self, f, Char);
        fmt_gast_case!(self, f, Int);
        fmt_gast_case!(self, f, Uint);
        fmt_gast_case!(self, f, Float);
        fmt_gast_case!(self, f, Str);
        fmt_gast_case!(self, f, Sym);
        fmt_gast_case!(self, f, Pair);
        fmt_gast_case!(self, f, List);
        unreachable!()
    }
}
