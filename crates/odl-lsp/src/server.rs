use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer };

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
}

pub fn capabilities() -> ServerCapabilities {
    let document_symbol_provider = Some(OneOf::Left(true));

    let text_document_sync = {
        let options = TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::FULL),
            ..Default::default()
        };
        Some(TextDocumentSyncCapability::Options(options))
    };

    ServerCapabilities {
        text_document_sync,
        document_symbol_provider,
        ..Default::default()
    }
}



#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let capabilities = capabilities();
        Ok(InitializeResult {
            capabilities,
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;

    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let message = format!("opened document: {}", params.text_document.uri.as_str());
        self.client.log_message(MessageType::INFO, message).await;
        
        let result = crate::handler::did_open(params.text_document.text);

        if let Err(error) = result {
            let mut diagnostics = vec![];
            diagnostics.push(crate::handler::error_to_diagnostic(error));
            self.client.publish_diagnostics(
                params.text_document.uri,
                diagnostics,
                Some(params.text_document.version)
            ).await;
        }
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(
                MarkedString::String("You're hovering!".to_string())
            ),
            range: None
        }))
    }
}

