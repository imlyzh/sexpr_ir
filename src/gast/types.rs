use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use super::Handle;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ConstantType {
    Nil,
    Bool,
    Char,
    Int,
    Uint,
    Float,
    Str,
    Sym,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum SimpleType {
    Const(ConstantType),
    Pair,
    List,
}

pub trait GetSimpleType {
    fn get_simple_type(&self) -> SimpleType;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum GAstType {
    Const(ConstantType),
    Pair(Handle<GAstType>, Handle<GAstType>),
    Tuple(Vec<GAstType>),
    List(Handle<GAstType>),
    Union(Vec<GAstType>)
}

impl GAstType {
    pub fn zip(&self) -> Self {
        match self {
            GAstType::Pair(x, y) =>
                Self::Pair(
                    Handle::new(x.zip()),
                    Handle::new(y.zip())),
            GAstType::Tuple(x) => {
                let r: HashSet<GAstType> = x.iter().map(Self::zip).collect();
                Self::List(Handle::new(GAstType::Union(r.into_iter().collect())))
            }
            _ => self.clone()
        }
    }
}

pub trait GetType {
    fn get_type(&self) -> GAstType;
}