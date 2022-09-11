use crate::expression::*;
use crate::serialization::*;
use crate::token::Span;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiseDeclaration {
    symbol: String,
    value: Option<Expression>,
    fields: Vec<ChoiseDeclaration>,
    span: Span,
}

impl Serializable for ChoiseDeclaration {
    fn serialize(
        &self,
        f: &mut Formatter,
        ctx: &SerializationContext,
    ) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.symbol)?;
        if self.value.as_ref().is_some() {
            write!(f, " = ")?;
            self.value.as_ref().unwrap().serialize(f, ctx)?;
        }
        write!(f, "\n")?;
        for field in &self.fields {
            field.serialize(f, &ctx.indented())?;
        }
        return Ok(());
    }
}

impl Display for ChoiseDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        return self.serialize(f, &SerializationContext::new());
    }
}

impl ChoiseDeclaration {
    pub fn new(symbol: String, exp: Option<Expression>, span: Span) -> ChoiseDeclaration {
        let fields = Vec::new();
        let value = None;
        return ChoiseDeclaration {
            symbol,
            value,
            fields,
            span,
        };
    }

    pub fn new_from_exp(symbol: String, exp: Expression, span: Span) -> ChoiseDeclaration {
        let fields = Vec::new();
        let value = Some(exp);
        return ChoiseDeclaration {
            symbol,
            value,
            fields,
            span,
        };
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    pub fn symbol(&self) -> &String {
        return &self.symbol;
    }

    pub fn get_fields_mut(&mut self) -> &mut Vec<ChoiseDeclaration> {
        return (self.fields).as_mut();
    }

    pub fn get_fields(&self) -> &Vec<ChoiseDeclaration> {
        return &self.fields;
    }

    pub fn get_field(&self, i: usize) -> Option<&ChoiseDeclaration> {
        return self.get_fields().get(i);
    }

    pub fn get_field_mut(&mut self, i: usize) -> Option<&mut ChoiseDeclaration> {
        return self.get_fields_mut().get_mut(i);
    }
}
