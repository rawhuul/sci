mod watcher;

use git2::Repository;
use std::{env, format, path::PathBuf};
use tokio::time::Duration;

use watcher::Watcher;

fn open_repo(repo_path: &PathBuf) -> Option<Repository> {
    Repository::open(repo_path).ok()
}

fn get_full_path(repo: &PathBuf) -> String {
    let absolute_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(&repo);

    match absolute_path.canonicalize() {
        Ok(path) => path.to_string_lossy().replace(r"\\?\", ""),
        Err(_) => format!("{repo:?}"),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("[ERROR]: Destination to repository must be passed.");
        return;
    }

    let repo_path = PathBuf::from(&args[1]);
    let repo = open_repo(&repo_path);

    if repo.is_none() {
        println!("[ERROR]: {} is not a Git Repo.", get_full_path(&repo_path));
        return;
    }

    let mut watch_dog = Watcher::new(repo.unwrap()).unwrap();

    let mut interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        let changed = watch_dog.watch();

        println!("{:?}\n{:?}\n", watch_dog, changed.unwrap().unwrap());
    }
}
