use std::fmt::Display;
use serde::{Serialize, Deserialize};

use super::GAst;


#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Pair(pub GAst, pub GAst);

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} . {})", self.0, self.1)
    }
}