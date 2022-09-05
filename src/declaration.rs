use crate::constant::*;
use crate::expression::*;
use crate::option::*;
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
        indents: usize,
    ) -> Result<(), ::std::fmt::Error> {
        return match self {
            Declaration::Opt(dec) => dec.serialize(f, indents),
            Declaration::Const(dec) => dec.serialize(f, indents),
        };
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, 0);
    }
}
