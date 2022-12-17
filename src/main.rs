extern crate sndfile;
extern crate dotenv;
extern crate tokio;
mod rubberband;
mod limiter;


use tokio::net::TcpSocket;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;
use std::thread;
use dotenv::dotenv;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = std::env::var("HOST").expect("HOST must be set");
    let listener = TcpListener::bind(&host).await?;
    println!("Listening on {}", host);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted connection from: {}", socket.peer_addr()?);
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }

}
