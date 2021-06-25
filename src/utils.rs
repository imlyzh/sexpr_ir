use std::collections::{HashMap, VecDeque};
use std::{iter::FromIterator, sync::Mutex};

use lazy_static::lazy_static;

use crate::gast::Handle;


lazy_static! {
    static ref GLOBAL_INTERN_STRING_POOL: Mutex<HashMap<Handle<String>, Handle<String>>> =
        Mutex::new(HashMap::new());
}

// fast(xD
#[macro_export]
macro_rules! fast_return {
	($e:expr) => {
		if let Ok(res) = $e {
			return Ok(res);
		}
	};
}

#[inline]
pub fn string_intern(i: &str) -> Handle<String> {
    let k = Handle::new(i.to_string());
    {
        if let Some(x) = GLOBAL_INTERN_STRING_POOL.lock().unwrap().get(&k) {
            return x.clone();
        }
    }
    GLOBAL_INTERN_STRING_POOL
        .lock()
        .unwrap()
        .insert(k.clone(), k.clone());
    k
}

#[inline]
pub fn escape_char(i: char) -> char {
    match i {
        '\\' => '\\',
        '\"' => '\"',
        '\'' => '\'',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => unreachable!(),
    }
}

#[inline]
pub fn state_machine(
    (mut prev, mut is_escape): (VecDeque<char>, bool),
    item: char,
) -> (VecDeque<char>, bool) {
    if is_escape {
        prev.push_back(escape_char(item));
        return (prev, false);
    }
    if item == '\\' {
        is_escape = true;
    } else {
        prev.push_back(item);
        is_escape = false;
    }
    (prev, is_escape)
}

#[inline]
pub fn escape_str(i: &str) -> String {
    let (char_string, is_escape) = i.chars().fold((VecDeque::new(), false), state_machine);
    assert_eq!(is_escape, false);
    String::from_iter(char_string.iter())
}

/*
#[inline]
pub fn str2char(i: &str) -> char {
    i.chars().next().unwrap()
}

#[inline]
pub fn match_error(keyword: &Handle<Symbol>) -> RuntimeError {
	RuntimeError::SyntaxError(
		SyntaxMatchError::SyntaxMatchError(keyword.clone()))
}
*/

#[macro_export]
macro_rules! impl_is_type {
    ($name:ident, $item:ident) => {
        pub fn $name(&self) -> bool {
            if let Self::$item(_) = self {
                true
            } else {
                false
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_item {
    ($name:ident, $item:ident, $tp:path) => {
        pub fn $name(&self) -> Option<$tp> {
            if let Self::$item(x) = self {
                Some(x.clone())
            } else {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! fmt_gast_case {
    ($self:ident, $f:ident, $item:ident) => {
        if let Self::$item(x) = $self {
            return $f.write_str(&x.to_string());
        }
    };
}

#[macro_export]
macro_rules! impl_get_constant_type {
    ($self:ident, $item:ident) => {
        if let Self::$item(_) = $self {
            return ConstantType::$item;
        }
    };
}

#[macro_export]
macro_rules! impl_get_simple_type {
    ($self:ident, $item:ident) => {
        if let Self::$item(_) = $self {
            return SimpleType::$item;
        }
    };
}
