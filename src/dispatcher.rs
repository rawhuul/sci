use std::{net::SocketAddr, path::PathBuf};
use tokio::{
    io::AsyncBufReadExt,
    net::{TcpListener, TcpStream},
};

use crate::runner::TestRunner;
use crate::utils::Utils;
use crate::watcher::Changes;

pub struct Dispatcher {
    dir: PathBuf,
    addr: SocketAddr,
    key: u128,
}

impl Dispatcher {
    pub fn new(dir: &PathBuf, addr: SocketAddr, key: u128) -> Self {
        Dispatcher {
            dir: dir.to_owned(),
            addr,
            key,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        println!(
            "Dispatcher listening to '{}' on '{}'",
            Utils::get_full_path(&self.dir),
            self.addr
        );

        while let Ok((stream, _)) = listener.accept().await {
            let d = self.dir.clone();
            let k = self.key;

            tokio::spawn(async move {
                if let Err(err) = Self::handle_client(stream, k, &d).await {
                    eprintln!("Error handling client: {}", err);
                }
            });
        }

        Ok(())
    }

    async fn handle_client(
        stream: TcpStream,
        key: u128,
        dir: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = tokio::io::BufReader::new(stream);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }

            Self::handle_message(line.trim(), key, dir);

            line.clear();
        }

        Ok(())
    }

    fn handle_message(message: &str, key: u128, dir: &PathBuf) {
        if let Some((k, changes)) = message.split_once(":") {
            if k.parse::<u128>().unwrap() == key {
                if let Ok(changes) = serde_json::from_str::<Changes>(&changes) {
                    // Perform your desired action based on the received changes
                    println!("Received: {:?}", changes);
                    // Perform your specific action here
                    let mut runner = TestRunner::new(dir);
                    let info = runner.identify();

                    println!("{:?}", info);
                }
            } else {
                eprintln!("Unverified Message Received! Discarding..");
            }
        } else {
            eprintln!("Invalid message format: {}", message);
        }
    }
}
