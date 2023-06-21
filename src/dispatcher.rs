use crate::watcher::Changes;

use md5::Digest;
use std::{format, net::SocketAddr, time::Duration, todo};
use tokio::{
    io::AsyncBufReadExt,
    net::{TcpListener, TcpStream},
    time::sleep,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

pub struct Dispatcher {
    addr: SocketAddr,
    msg_key: Digest,
}

impl Dispatcher {
    pub fn new(addr: SocketAddr, msg_key: Digest) -> Self {
        Dispatcher { addr, msg_key }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        println!("Dispatcher listening on {}", self.addr);

        while let Ok((stream, _)) = listener.accept().await {
            let k = self.msg_key.clone();

            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(stream, &k).await {
                    eprintln!("Error handling client: {}", err);
                }
            });
        }

        Ok(())
    }

    async fn handle_client(
        stream: TcpStream,
        msg_key: &Digest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = tokio::io::BufReader::new(stream);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }

            Self::handle_message(line.trim().to_string(), msg_key);

            line.clear();
        }

        Ok(())
    }

    fn handle_message(message: String, msg_key: &Digest) {
        if let Some((key, changes)) = message.split_once(":") {
            if key == format!("{msg_key:?}") {
                if let Ok(changes) = serde_json::from_str::<Changes>(&changes) {
                    // Perform your desired action based on the received changes
                    println!("Received: {:?}", changes);
                    // Perform your specific action here
                }
            } else {
                eprintln!("Unverified Message Recieved! Discarding..");
            }
        } else {
            eprintln!("Invalid message format: {}", message);
        }
    }
}
