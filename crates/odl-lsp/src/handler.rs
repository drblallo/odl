use odl::parser::*;
use odl::error::ParserError;
use std::error::Error;
use tower_lsp::lsp_types::*;

pub fn did_open(document: String) -> Result<(), ParserError> {
    let mut lexer = Parser::new(&document)?;
    lexer.declaration()?;
    Ok(())
}

pub fn error_to_diagnostic(error: ParserError) -> Diagnostic {
    let (start, end) = match error {
        ParserError::EndOfTokenStream(_) => {
            //TODO: Get end of file/stream position
            let start = Position::new(0,0);
            let end = Position::new(0,0);
            (start, end)
        }
        ParserError::Indentation(ref err) => {
            let start = Position::new(err.span.lo.row, err.span.lo.column);
            let end = Position::new(err.span.hi.row, err.span.hi.column);
            (start, end)
        }    
        ParserError::UnexpectedToken(ref err) => {
            let token_span = err.token.span;
            let start = Position::new(token_span.lo.row, token_span.lo.column);
            let end = Position::new(token_span.hi.row, token_span.hi.column);
            (start, end)
        }
    };

    Diagnostic {
        range: Range::new(start,end),
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("odl".to_string()),
        message: error.description().to_string(),
        related_information: None,
        tags: None,
        data: None
    }
}