use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the socket
    let socket = TcpStream::connect("127.0.0.1:5555").await?;
    let (mut reader, mut writer) = io::split(socket);

    // Spawn a task to read from the socket
    let socket_task = tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let n = match reader.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; error = {:?}", e);
                    return;
                }
            };
            println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
        }
    });

    // Read from the terminal
    let terminal_task = tokio::spawn(async move {
        let mut reader = BufReader::new(io::stdin());
        loop {
            let mut input = String::new();
            print!("Enter a message (or 'quit' to exit): ");
            io::stdout().flush().await.unwrap();
            reader.read_line(&mut input).await.unwrap();
            let input = input.trim();
            if input == "quit" {
                break;
            }
            println!("You entered: {}", input);

            writer.write_all(input.as_bytes()).await.unwrap();
        }
    });

    // Wait for both tasks to finish
    let _ = tokio::try_join!(socket_task, terminal_task);
    Ok(())
}
