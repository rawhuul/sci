mod dispatcher;
mod watcher;

use argh::FromArgs;
use git2::Repository;
use serde_json;
use std::net::SocketAddr;
use std::{path::PathBuf, thread};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use dispatcher::Dispatcher;
use watcher::Watcher;

#[derive(FromArgs)]
/// A simple CI system.
struct Args {
    /// destination to repository
    #[argh(positional)]
    repo_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();

    let repo_path = args.repo_path;
    let repo = Repository::open(&repo_path)?;
    let mut watcher = Watcher::new(repo)?;

    let addr = "127.0.0.1:8080".parse()?;
    let dispatcher = Dispatcher::new(addr);

    // Spawn a Tokio task to run the dispatcher asynchronously
    tokio::spawn(async move {
        if let Err(err) = dispatcher.start().await {
            eprintln!("Dispatcher error: {}", err);
        }
    });

    // Spawn a Tokio task to run the watcher asynchronously
    tokio::spawn(async move {
        loop {
            if let Ok(changed) = watcher.watch() {
                if let Some(change) = changed {
                    let msg = serde_json::to_string(&change).unwrap();
                    // println!("{msg}");
                    if let Err(err) = send_change_to_dispatcher(&addr, &msg).await {
                        eprintln!("Error sending message: {}", err);
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    // Keep the main thread alive
    loop {
        // Perform other operations if needed
        thread::park();
    }
}

async fn send_change_to_dispatcher(
    addr: &SocketAddr,
    msg: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(msg.as_bytes()).await?;
    Ok(())
}
