use core::fmt::Formatter;

pub fn indent(f: &mut std::fmt::Formatter, indent: usize) -> Result<(), std::fmt::Error> {
    for _ in 0..indent {
        write!(f, " ")?;
    }
    return Ok(());
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmissionKind {
    Non,
    Opt,
    Const,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SerializationContext {
    pub indent: usize,
    pub emission_kind: EmissionKind,
}

impl SerializationContext {
    pub fn new() -> SerializationContext {
        SerializationContext {
            indent: 0,
            emission_kind: EmissionKind::Non,
        }
    }

    pub fn indented(&self) -> SerializationContext {
        let mut new_context = self.clone();
        new_context.indent = new_context.indent + 1;
        return new_context;
    }

    pub fn emitting_option(&self) -> SerializationContext {
        let mut new_context = self.clone();
        new_context.emission_kind = EmissionKind::Opt;
        return new_context;
    }

    pub fn emitting_const(&self) -> SerializationContext {
        let mut new_context = self.clone();
        new_context.emission_kind = EmissionKind::Const;
        return new_context;
    }
}

pub trait Serializable {
    fn serialize(
        &self,
        f: &mut Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error>;
}
