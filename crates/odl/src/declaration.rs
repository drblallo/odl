use crate::alternative::*;
use crate::choice::*;
use crate::constant::*;
use crate::error::*;
use crate::option::*;
use crate::serialization::*;
use crate::symbol_table::*;
use crate::symbol_table::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Opt(OptionDeclaration),
    Const(ConstantDeclaration),
    Alt(AlternativeDeclaration),
    Choice(ChoiseDeclaration),
}

impl Serializable for Declaration {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        return match self {
            Declaration::Choice(dec) => dec.serialize(f, ctx),
            Declaration::Opt(dec) => dec.serialize(f, ctx),
            Declaration::Const(dec) => dec.serialize(f, ctx),
            Declaration::Alt(dec) => dec.serialize(f, ctx),
        };
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}

impl Declaration {
    pub fn name(&self) -> String {
        return match self {
            Declaration::Choice(_) => "".to_owned(),
            Declaration::Opt(dec) => dec.name().to_owned(),
            Declaration::Const(dec) => dec.name().to_owned(),
            Declaration::Alt(dec) => dec.name().to_owned(),
        };
    }

    pub fn is_choise(&self) -> bool {
        return match self {
            Declaration::Choice(_) => true,
            _ => false,
        };
    }

    pub fn type_check(&self, table: &SymbolTable) -> Result<(), ParserError> {
        return match self {
            Declaration::Choice(dec) => dec.type_check(table),
            Declaration::Opt(dec) => dec.type_check(table),
            Declaration::Const(dec) => dec.type_check(table),
            Declaration::Alt(dec) => dec.type_check(table),
        };
    }
}
