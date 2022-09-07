use crate::constant::*;
use crate::option::*;
use crate::serialization::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Opt(OptionDeclaration),
    Const(ConstantDeclaration),
}

impl Serializable for Declaration {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        return match self {
            Declaration::Opt(dec) => dec.serialize(f, ctx),
            Declaration::Const(dec) => dec.serialize(f, ctx),
        };
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}
