use std::net::SocketAddr;
use tokio::{
    io::AsyncBufReadExt,
    net::{TcpListener, TcpStream},
};

use crate::watcher::Changes;

pub struct Dispatcher {
    addr: SocketAddr,
    key: u128,
}

impl Dispatcher {
    pub fn new(addr: SocketAddr, key: u128) -> Self {
        Dispatcher { addr, key }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        println!("Dispatcher listening on {}", self.addr);

        while let Ok((stream, _)) = listener.accept().await {
            let k = self.key;

            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(stream, k).await {
                    eprintln!("Error handling client: {}", err);
                }
            });
        }

        Ok(())
    }

    async fn handle_client(stream: TcpStream, key: u128) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = tokio::io::BufReader::new(stream);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }

            Self::handle_message(line.trim(), key);

            line.clear();
        }

        Ok(())
    }

    fn handle_message(message: &str, key: u128) {
        if let Some((k, changes)) = message.split_once(":") {
            if k.parse::<u128>().unwrap() == key {
                if let Ok(changes) = serde_json::from_str::<Changes>(&changes) {
                    // Perform your desired action based on the received changes
                    println!("Received: {:?}", changes);
                    // Perform your specific action here
                }
            } else {
                eprintln!("Unverified Message Received! Discarding..");
            }
        } else {
            eprintln!("Invalid message format: {}", message);
        }
    }
}
