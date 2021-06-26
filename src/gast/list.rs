use std::fmt::Display;
use serde::{Serialize, Deserialize};

use super::GAst;


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct List(pub Vec<GAst>, pub Option<GAst>);

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.0.iter();
        let str_list: Vec<String> = list.map(GAst::to_string).collect();
        let str_pair_right = if let Some(x) = self.1.clone() {
            format!(" . {}", x)
        } else {
            "".to_string()
        };
        write!(f, "({}{})", str_list.join(" "), str_pair_right)
    }
}
