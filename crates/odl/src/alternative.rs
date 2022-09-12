use crate::error::*;
use crate::option::*;
use crate::serialization::*;
use crate::symbol_table::*;
use crate::token::Span;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct AlternativeDeclaration {
    name: String,
    alternatives: Vec<OptionDeclaration>,
    span: Span,
}

impl Serializable for AlternativeDeclaration {
    fn serialize(
        &self,
        f: &mut Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        indent(f, ctx.indent)?;
        write!(f, "alt {}\n", self.name)?;
        for decl in &self.alternatives {
            decl.serialize(f, &ctx.indented().emitting_option())?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Display for AlternativeDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}

impl AlternativeDeclaration {
    pub fn new(name: String, span: Span) -> AlternativeDeclaration {
        let alternatives = Vec::new();
        return AlternativeDeclaration {
            name,
            alternatives,
            span,
        };
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn get_fields_mut(&mut self) -> &mut Vec<OptionDeclaration> {
        return (self.alternatives).as_mut();
    }

    pub fn get_fields(&self) -> &Vec<OptionDeclaration> {
        return &self.alternatives;
    }

    pub fn get_field(&self, i: usize) -> Option<&OptionDeclaration> {
        return self.get_fields().get(i);
    }

    pub fn get_field_mut(&mut self, i: usize) -> Option<&mut OptionDeclaration> {
        return self.get_fields_mut().get_mut(i);
    }

    pub fn type_check(&self, table: &SymbolTable) -> Result<(), ParserError> {
        Ok(())
    }
}
