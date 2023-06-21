mod dispatcher;
mod utils;
mod watcher;

use argh::FromArgs;
use git2::Repository;
use std::{path::PathBuf, thread};

use dispatcher::Dispatcher;
use utils::Utils;
use watcher::Watcher;

#[derive(FromArgs)]
/// A simple CI system.
struct Args {
    /// destination to repository
    #[argh(positional)]
    repo_path: PathBuf,
    /// port address for the dispatcher (default: 8080)
    #[argh(option, short = 'p', default = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();

    let repo = Repository::open(&args.repo_path)?;
    let mut watcher = Watcher::new(repo)?;

    let addr = format!("127.0.0.1:{}", args.port).parse()?;
    let utils = Utils::new(addr);

    let dispatcher = Dispatcher::new(addr, utils.key());

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
                    if let Err(err) = utils.send_msg(&msg).await {
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
