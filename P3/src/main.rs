use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_request(
    mut socket: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buf: [u8; 1024]= [0; 1024];

    loop {
        let n =socket.read(&mut buf).await?;
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
        if parts.len() < 3 { return Ok(()) }

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

        if method == "GET" {
            println!("Server: get");
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
            length = body.len(), // OBS: len() i byte; for enkel ASCII/vanlig tekst er dette ok
            body = body
        );
        socket.write_all(response.as_bytes()).await?;
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener= tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server listening for connection");
    loop {
        let(socket, _) = listener.accept().await?;
        println!("Server Connection from {}", socket.peer_addr()?);

        tokio::spawn(async move {
            if let Err(e) = handle_request(socket).await {
                eprintln!("Server error: {}", e);
            }

        });
    }
}

// cmd command for adding entire messages:
/*
powershell -Command "$c = New-Object Net.Sockets.TcpClient('127.0.0.1',3000); $s = $c.GetStream(); $w = New-Object IO.StreamWriter($s); $w.AutoFlush = $true; while($true){ $line = Read-Host '>'; if($line -eq 'quit'){ break }; $w.WriteLine($line) } $c.Close()"
 */
