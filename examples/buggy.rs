use std::fmt::Display;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    InitializeResult, MessageType, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};
use tower_lsp::Client;
use tower_lsp::{lsp_types::InitializeParams, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self { client }
    }

    async fn log<D: Display>(&self, message: D) {
        self.client.log_message(MessageType::LOG, message).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        self.log("LOG 1").await;
        self.log("LOG 2").await;
        self.log("LOG 3").await;
        self.log("LOG 4").await;
        self.log("LOG 5").await;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let (mut stdin, mut stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let service_builder = LspService::build(|client| Backend::new(client));
    let (service, socket) = service_builder.finish();

    Server::new(&mut stdin, &mut stdout, socket)
        .serve(service)
        .await;
}
