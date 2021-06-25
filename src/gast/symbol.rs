use std::{fmt::Display, hash::Hash};
use serde::{Serialize, Deserialize};

use super::Handle;
use crate::utils::string_intern;


#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Localation {
    pub line: usize,
    pub colum: usize,
    pub pos: usize,
}

impl Localation {
    pub fn new(line: usize, colum: usize, pos: usize) -> Self {
        Self { line, colum, pos }
    }
}


#[derive(Debug, Clone, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Symbol (pub Handle<String>, pub Localation);

impl Symbol {
    pub fn new(i: &str) -> Self {
        Symbol (string_intern(i), Localation::default())
    }

    pub fn from(i: &str, pos: &Localation) -> Self {
        Symbol (string_intern(i), pos.clone())
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
