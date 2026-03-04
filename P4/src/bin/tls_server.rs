use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile;
use rcgen::generate_simple_self_signed;

async fn handle_request<S>(mut stream: S) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    S: AsyncReadExt + AsyncWriteExt + Unpin + Send + 'static,
{
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            println!("Server: Client closed connection");
            return Ok(());
        }

        let message = String::from_utf8_lossy(&buf[0..n]);
        let request_str = message.to_string();
        let mut lines = request_str.lines();
        let request_line = match lines.next() {
            Some(line) => line,
            None => {
                println!("Server: invalid (empty) request");
                return Ok(());
            }
        };

        println!("Server: received request {}", request_line);

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Ok(());
        }

        let method = parts[0];
        let path = parts[1];
        let _version = parts[2];

        println!("Server: method {}", method);
        println!("Server: path {}", path);
        println!("Server: version {}", _version);

        if request_line == "exit" {
            println!("Server: Closing connection");
            return Ok(());
        }

        let body = match path {
            "/" => "<html><body><h1>Hovedside</h1>\
            <p>Velkommen!</p>\
            <a href=\"/page1\">Page 1</a><br>\
            <a href=\"/page2\">Page 2</a>\
            </body></html>",
            "/page1" => "<html><body><h1>Side 1</h1><p>Dette er page1</p></body></html>",
            "/page2" => "<html><body><h1>Side 2</h1><p>Dette er page2</p></body></html>",
            _ => "<html><body><h1>404</h1><p>Siden finnes ikke</p></body></html>",
        };

        let status_line = if path == "/" || path == "/page1" || path == "/page2" {
            "HTTP/1.1 200 OK"
        } else {
            "HTTP/1.1 404 Not Found"
        };

        let response = format!(
            "{status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {length}\r\n\r\n{body}",
            status = status_line,
            length = body.len(),
            body = body
        );

        stream.write_all(response.as_bytes()).await?;

        // For simplicity, close after responding once (like typical HTTP/1.0 behavior).
        return Ok(());
    }
}

fn make_rustls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    // Generate a self-signed certificate for localhost
    let cert = generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_pem = cert.serialize_pem()?;
    let key_pem = cert.serialize_private_key_pem();

    // Parse PEM into rustls types
    let certs = {
        let mut pem = cert_pem.as_bytes();
        let mut reader = std::io::BufReader::new(&mut pem);
        let certs = rustls_pemfile::certs(&mut reader)?;
        certs.into_iter().map(Certificate).collect()
    };

    let mut key_reader = std::io::BufReader::new(key_pem.as_bytes());
    let keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)?;
    if keys.is_empty() {
        return Err("No private keys found".into());
    }
    let priv_key = PrivateKey(keys[0].clone());

    let config = ServerConfig::builder()
      // pick sensible protocol/cipher defaults
        .with_safe_defaults()
      // Server do not require client certificate
        .with_no_client_auth()
      // attaches certificate and key so tls can perform handshake
        .with_single_cert(certs, priv_key)?;

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build TLS config and acceptor
    // Creates a fresh keypair and certificate in memory
    let tls_config = make_rustls_config()?;
    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let listener = TcpListener::bind("127.0.0.1:8443").await?;
    println!("TLS server listening on https://127.0.0.1:8443");

    loop {
        // TCP socket established
        let (socket, addr) = listener.accept().await?;
        println!("Connection from {}", addr);
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            // Handshake begins
            // sends clientHello, ServerHello
            // send certificate
            // send certificate verify, with a private key signature
            // Client validates the certificate and the signature
            match acceptor.accept(socket).await {
                Ok(tls_stream) => {
                    println!("TLS handshake success from {}",&addr);
                    if let Err(e) = handle_request(tls_stream).await {
                        eprintln!("Connection error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("TLS handshake failed: {}", e);
                }
            }
        });
    }
}