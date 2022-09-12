use odl_lsp::server::*;
use tower_lsp::{LspService, Server};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {

    let mut args = std::env::args();
    let stream = match args.nth(1).as_deref() {
        None => {
            // If no argument is supplied (args is just the program name), then
            // we presume that the client has opened the TCP port and is waiting
            // for us to connect. This is the connection pattern used by clients
            // built with vscode-langaugeclient.
            TcpStream::connect("127.0.0.1:9257").await.unwrap()
        }
        Some("--listen") => {
            // If the `--listen` argument is supplied, then the roles are
            // reversed: we need to start a server and wait for the client to
            // connect.
            let listener = TcpListener::bind("127.0.0.1:9257").await.unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            stream
        }
        Some(arg) => panic!(
            "Unrecognized argument: {}. Use --listen to listen for connections.",
            arg
        ),
    };

    let (read, write) = tokio::io::split(stream);

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(read, write, socket).serve(service).await;}
