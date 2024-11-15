use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

use super::Handle;
// use crate::utils::string_intern;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub path: Handle<String>,
    pub line: usize,
    pub colum: usize,
    pub pos: usize,
}

impl Location {
    pub fn new(path: Handle<String>, line: usize, colum: usize, pos: usize) -> Self {
        Self {
            path,
            line,
            colum,
            pos,
        }
    }
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Symbol(pub Handle<String>, pub Location);

impl Symbol {
    pub fn new(i: &str) -> Self {
        Symbol(Handle::new(i.to_owned()), Location::default())
    }

    pub fn from(i: &str, pos: &Location) -> Self {
        Symbol(Handle::new(i.to_owned()), pos.clone())
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

unsafe impl Sync for Symbol {}
unsafe impl Send for Symbol {}
