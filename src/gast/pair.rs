use std::fmt::Display;
use serde::{Serialize, Deserialize};

use super::{GAst, Handle, types::{GAstType, GetType}};


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Pair(pub GAst, pub GAst);

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} . {})", self.0, self.1)
    }
}

impl GetType for Pair {
    fn get_type(&self) -> GAstType {
        GAstType::Pair(
            Handle::new(self.0.get_type()),
            Handle::new(self.1.get_type()))
    }
}