use crate::declaration::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable<'a> {
    symbols: HashMap<String, &'a Declaration>,
    parent: Option<&'a SymbolTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        return SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        };
    }

    pub fn new_from_parent(parent: &'a SymbolTable) -> SymbolTable<'a> {
        return SymbolTable {
            symbols: HashMap::new(),
            parent: Some(parent),
        };
    }

    pub fn make_child(&'a self) -> SymbolTable<'a> {
        return SymbolTable {
            symbols: HashMap::new(),
            parent: Some(self),
        };
    }

    pub fn insert(&mut self, dec: &'a Declaration) {
        self.symbols.insert(dec.name(), dec);
    }

    pub fn get(&self, s: &String) -> Option<&Declaration> {
        let symb = self.symbols.get(s);
        if symb.is_some() {
            return Some(*symb.unwrap());
        }
        if self.parent.is_none() {
            return None;
        }
        return self.parent.unwrap().get(s);
    }
}
