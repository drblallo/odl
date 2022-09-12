use crate::declaration::*;
use crate::error::*;
use crate::serialization::*;
use crate::symbol_table::*;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Document {
    pub entries: Vec<Declaration>,
}

impl Serializable for Document {
    fn serialize(
        &self,
        f: &mut Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        for entry in &self.entries {
            entry.serialize(f, ctx)?;
        }
        Ok(())
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}

impl Document {
    pub fn new() -> Document {
        return Document {
            entries: Vec::new(),
        };
    }

    pub fn type_check(&self) -> Result<(), ParserError> {
        let mut table = SymbolTable::new();
        for decl in &self.entries {
            if !decl.is_choise() {
                table.insert(&decl);
            }
        }

        for decl in &self.entries {
            decl.type_check(&table)?;
        }
        Ok(())
    }
}
