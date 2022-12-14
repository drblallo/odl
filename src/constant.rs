use crate::error::*;
use crate::expression::*;
use crate::serialization::*;
use crate::symbol_table::*;
use crate::token::Span;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantBody {
    Direct(Expression),
    Content(Vec<ConstantDeclaration>),
}

impl Serializable for ConstantBody {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        match self {
            ConstantBody::Direct(e) => {
                write!(f, " = ")?;
                e.serialize(f, ctx)?;
            }
            ConstantBody::Content(vec) => {
                write!(f, "\n")?;

                for entry in vec {
                    entry.serialize(f, &ctx.indented())?;
                }
            }
        };
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDeclaration {
    name: String,
    body: ConstantBody,
    span: Span,
}

impl ConstantDeclaration {
    pub fn new_direct(name: String, initializer: Expression, span: Span) -> ConstantDeclaration {
        let body = ConstantBody::Direct(initializer);
        return ConstantDeclaration { name, body, span };
    }

    pub fn new(name: String, span: Span) -> ConstantDeclaration {
        let body = ConstantBody::Content(Vec::new());
        return ConstantDeclaration { name, body, span };
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    fn has_initializer(&self) -> bool {
        return match self.body {
            ConstantBody::Direct(_) => true,
            ConstantBody::Content(_) => false,
        };
    }

    fn has_children(&self) -> bool {
        return match self.body {
            ConstantBody::Direct(_) => false,
            ConstantBody::Content(_) => true,
        };
    }

    pub fn get_initializer(&self) -> Option<&Expression> {
        return match &self.body {
            ConstantBody::Direct(ref exp) => Some(exp),
            ConstantBody::Content(_) => None,
        };
    }

    pub fn get_fields_mut(&mut self) -> Option<&mut Vec<ConstantDeclaration>> {
        return match &mut self.body {
            ConstantBody::Direct(_) => None,
            ConstantBody::Content(ref mut vec) => Some(vec),
        };
    }

    pub fn get_fields(&self) -> Option<&Vec<ConstantDeclaration>> {
        return match &self.body {
            ConstantBody::Direct(_) => None,
            ConstantBody::Content(ref vec) => Some(vec),
        };
    }

    pub fn get_field(&self, i: usize) -> Option<&ConstantDeclaration> {
        return match self.get_fields() {
            None => None,
            Some(ref children) => Some(&children[i]),
        };
    }

    pub fn get_field_mut(&mut self, i: usize) -> Option<&mut ConstantDeclaration> {
        return self.get_fields_mut().and_then(|x| x.get_mut(i));
    }

    pub fn type_check(&self, table: &SymbolTable) -> Result<(), ParserError> {
        Ok(())
    }
}

impl Serializable for ConstantDeclaration {
    fn serialize(
        &self,
        f: &mut std::fmt::Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        indent(f, ctx.indent)?;
        if ctx.emission_kind != EmissionKind::Const {
            write!(f, "const {}", self.name)?;
        } else {
            write!(f, "{}", self.name)?;
        }
        return self.body.serialize(f, &ctx.emitting_const());
    }
}

impl Display for ConstantDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}
