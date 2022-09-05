use crate::declaration::*;
use crate::expression::*;
use crate::token::Span;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct OptionDeclaration {
    name: String,
    fields: Box<Vec<Declaration>>,
    span: Span,
}

impl Serializable for OptionDeclaration {
    fn serialize(&self, f: &mut Formatter, indents: usize) -> Result<(), ::std::fmt::Error> {
        indent(f, indents)?;
        write!(f, "option {}\n", self.name)?;

        for decl in &*self.fields {
            decl.serialize(f, indents + 1)?;
        }

        return Ok(());
    }
}

impl Display for OptionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, 0);
    }
}

impl OptionDeclaration {
    pub fn new(name: String, span: Span) -> OptionDeclaration {
        let fields = Box::new(Vec::new());
        return OptionDeclaration { name, fields, span };
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn get_fields_mut(&mut self) -> &mut Vec<Declaration> {
        return (*self.fields).as_mut();
    }

    pub fn get_fields(&self) -> &Vec<Declaration> {
        return &*self.fields;
    }

    pub fn get_field(&self, i: usize) -> Option<&Declaration> {
        return self.get_fields().get(i);
    }

    pub fn get_field_mut(&mut self, i: usize) -> Option<&mut Declaration> {
        return self.get_fields_mut().get_mut(i);
    }
}
