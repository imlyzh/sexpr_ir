use std::fmt::Display;

use super::GAst;


#[derive(Debug, Clone, PartialEq)]
pub struct List(pub Vec<GAst>);


impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.0.iter();
        let str_list: Vec<String> = list.map(GAst::to_string).collect();
        write!(f, "({})", str_list.join(" "))
    }
}
