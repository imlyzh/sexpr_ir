use std::fmt::Display;

use super::Handle;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Constant {
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(Handle<String>),
}

impl Constant {
    impl_is_type!(is_bool, Bool);
    impl_is_type!(is_char, Char);
    impl_is_type!(is_int, Int);
    impl_is_type!(is_uint, Uint);
    impl_is_type!(is_float, Float);
    impl_is_type!(is_str, Str);
    pub fn is_nil(&self) -> bool {
		matches!(self, Self::Nil)
    }
}

impl Constant {
    impl_get_item!(get_bool, Bool, bool);
    impl_get_item!(get_char, Char, char);
    impl_get_item!(get_int, Int, i64);
    impl_get_item!(get_uint, Uint, u64);
    impl_get_item!(get_float, Float, f64);
    impl_get_item!(get_str, Str, Handle<String>);
    pub fn get_nil(&self) -> Option<()> {
        if self.is_nil() { Some(()) } else { None }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Nil = self {
            return f.write_str("nil");
        }
        fmt_gast_case!(self, f, Bool);
        fmt_gast_case!(self, f, Char);
        fmt_gast_case!(self, f, Int);
        fmt_gast_case!(self, f, Uint);
        fmt_gast_case!(self, f, Float);
        fmt_gast_case!(self, f, Str);
        unreachable!()
    }
}