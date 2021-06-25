use std::fmt::Display;
use serde::{Serialize, Deserialize};

use super::{GAst, types::{GAstType, GetType}};


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct List(pub Vec<GAst>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.0.iter();
        let str_list: Vec<String> = list.map(GAst::to_string).collect();
        write!(f, "({})", str_list.join(" "))
    }
}

impl GetType for List {
    fn get_type(&self) -> GAstType {
        GAstType::Tuple(self.0.iter().map(GAst::get_type).collect())
    }
}