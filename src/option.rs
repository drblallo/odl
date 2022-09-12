use crate::alternative::*;
use crate::constant::ConstantDeclaration;
use crate::error::*;
use crate::serialization::*;
use crate::symbol_table::*;
use crate::token::Span;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum OptionField {
    SubOption(OptionDeclaration),
    Const(ConstantDeclaration),
    Alt(AlternativeDeclaration),
}

impl Serializable for OptionField {
    fn serialize(
        &self,
        f: &mut Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        return match self {
            OptionField::SubOption(c) => c.serialize(f, ctx),
            OptionField::Const(c) => c.serialize(f, ctx),
            OptionField::Alt(c) => c.serialize(f, ctx),
        };
    }
}

impl Display for OptionField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionDeclaration {
    name: String,
    fields: Vec<OptionField>,
    span: Span,
}

impl Serializable for OptionDeclaration {
    fn serialize(
        &self,
        f: &mut Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        indent(f, ctx.indent)?;
        if ctx.emission_kind != EmissionKind::Opt {
            write!(f, "opt {}\n", self.name)?;
        } else {
            write!(f, "{}\n", self.name)?;
        }

        for decl in &self.fields {
            decl.serialize(f, &ctx.indented().emitting_option())?;
        }

        write!(f, "\n")?;

        return Ok(());
    }
}

impl Display for OptionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}

impl OptionDeclaration {
    pub fn new(name: String, span: Span) -> OptionDeclaration {
        let fields = Vec::new();
        return OptionDeclaration { name, fields, span };
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn get_fields_mut(&mut self) -> &mut Vec<OptionField> {
        return (self.fields).as_mut();
    }

    pub fn get_fields(&self) -> &Vec<OptionField> {
        return &self.fields;
    }

    pub fn get_field(&self, i: usize) -> Option<&OptionField> {
        return self.get_fields().get(i);
    }

    pub fn get_field_mut(&mut self, i: usize) -> Option<&mut OptionField> {
        return self.get_fields_mut().get_mut(i);
    }

    pub fn type_check(&self, table: &SymbolTable) -> Result<(), ParserError> {
        Ok(())
    }
}
