use md5::Digest;
use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::AsyncBufReadExt,
    net::{TcpListener, TcpStream},
};

use crate::watcher::Changes;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Dispatcher {
    addr: SocketAddr,
    msg_key: String,
}

impl Dispatcher {
    pub fn new(addr: SocketAddr, msg_key: Digest) -> Self {
        Dispatcher {
            addr,
            msg_key: format!("{msg_key:?}"),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        println!("Dispatcher listening on {}", self.addr);

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner} {msg}")
                .unwrap(),
        );
        spinner.set_message("Waiting for incoming connections");
        spinner.enable_steady_tick(Duration::from_millis(100));

        while let Ok((stream, _)) = listener.accept().await {
            let k = self.msg_key.clone();
            let spinner_clone = spinner.clone();

            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(stream, &k).await {
                    eprintln!("Error handling client: {}", err);
                }
                spinner_clone.set_message("Waiting for incoming connections");
            });
        }

        spinner.finish_and_clear();
        Ok(())
    }

    async fn handle_client(
        stream: TcpStream,
        msg_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = tokio::io::BufReader::new(stream);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }

            Self::handle_message(line.trim(), msg_key);

            line.clear();
        }

        Ok(())
    }

    fn handle_message(message: &str, msg_key: &str) {
        if let Some((key, changes)) = message.split_once(":") {
            if key == msg_key {
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
