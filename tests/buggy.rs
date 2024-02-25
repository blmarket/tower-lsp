use std::io::Cursor;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    InitializeResult, MessageType, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};
use tower_lsp::Client;
use tower_lsp::{lsp_types::InitializeParams, LanguageServer, LspService, Server};

const REQ1: &str = r#"{"jsonrpc":"2.0","method":"initialize","params":{"capabilities":{}},"id":1}"#;

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        self.client.log_message(MessageType::LOG, "LOG 0").await;
        self.client.log_message(MessageType::LOG, "LOG 1").await;
        self.client.log_message(MessageType::LOG, "LOG 2").await;
        
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

fn mock_request() -> Vec<u8> {
    format!("Content-Length: {}\r\n\r\n{}", REQ1.len(), REQ1).into_bytes()
}

fn mock_stdio() -> (Cursor<Vec<u8>>, Vec<u8>) {
    (Cursor::new(mock_request()), Vec::new())
}

#[tokio::test]
async fn test_logs() {
    let (mut stdin, mut stdout) = mock_stdio();

    let service_builder = LspService::build(|client| Backend { client });
    let (service, socket) = service_builder.finish();
    
    Server::new(&mut stdin, &mut stdout, socket)
        .serve(service)
        .await;
}
