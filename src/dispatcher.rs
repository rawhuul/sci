use crate::watcher::Changes;

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

pub struct Dispatcher {
    addr: SocketAddr,
}

impl Dispatcher {
    pub fn new(addr: SocketAddr) -> Self {
        Dispatcher { addr }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        println!("Dispatcher listening on {}", self.addr);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(stream).await {
                    eprintln!("Error handling client: {}", err);
                }
            });
        }

        Ok(())
    }

    async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let framed = FramedRead::new(stream, LinesCodec::new());

        // Process each line received from the client
        let mut lines = framed.map(|result| match result {
            Ok(line) => Self::handle_message(line),
            Err(e) => eprintln!("Error reading line: {}", e),
        });

        // Process messages as they arrive
        while let Some(_) = lines.next().await {
            // Add a log statement or print statement here to see the received messages
            // println!("Received message from client");
        }

        Ok(())
    }

    fn handle_message(message: String) {
        if let Ok(changes) = serde_json::from_str::<Changes>(&message) {
            // Perform your desired action based on the received changes
            println!("Received: {:?}", changes);
            // Perform your specific action here
            // ...
        } else {
            eprintln!("Invalid message format: {}", message);
        }
    }
}
