use std::fmt::Display;

use serde::{Serialize, Deserialize};

use super::{Handle, symbol::Symbol, types::{ConstantType, GAstType, GetSimpleType, GetType, SimpleType}};


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
    Sym(Handle<Symbol>),
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
        fmt_gast_case!(self, f, Sym);
        unreachable!()
    }
}

impl Constant {
    impl_is_type!(is_bool, Bool);
    impl_is_type!(is_char, Char);
    impl_is_type!(is_int, Int);
    impl_is_type!(is_uint, Uint);
    impl_is_type!(is_float, Float);
    impl_is_type!(is_str, Str);
    impl_is_type!(is_sym, Sym);
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
    impl_get_item!(get_sym, Sym, Handle<Symbol>);
    pub fn get_nil(&self) -> Option<()> {
        if self.is_nil() { Some(()) } else { None }
    }
}

impl Constant {
    pub fn get_constant_type(&self) -> ConstantType {
        impl_get_constant_type!(self, Bool);
        impl_get_constant_type!(self, Char);
        impl_get_constant_type!(self, Int);
        impl_get_constant_type!(self, Uint);
        impl_get_constant_type!(self, Float);
        impl_get_constant_type!(self, Str);
        impl_get_constant_type!(self, Sym);
        if let Self::Nil = self {
            return ConstantType::Nil;
        }
        unreachable!()
    }
}

impl GetSimpleType for Constant {
    fn get_simple_type(&self) -> SimpleType {
        SimpleType::Const(self.get_constant_type())
    }
}

impl GetType for Constant {
    fn get_type(&self) -> GAstType {
        GAstType::Const(self.get_constant_type())
    }
}
