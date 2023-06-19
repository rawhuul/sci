use git2::{Repository, StatusOptions};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("[ERROR]: Destination to repository must be passed.");
        return;
    }

    match Repository::open(&args[1]) {
        Ok(repo) => {
            let mut options = StatusOptions::new();
            options.include_untracked(true);
            options.include_ignored(false);

            if let Ok(statuses) = repo.statuses(Some(&mut options)) {
                let has_changes = !statuses.is_empty();
                println!("Repository has changes: {}", has_changes);

                if has_changes {
                    for entry in statuses.iter() {
                        if entry.status().is_index_new() {
                            if let Some(head) = repo.head().ok() {
                                if let Ok(commit) = head.peel_to_commit() {
                                    println!("Committed changes found:");
                                    println!("Commit ID: {}", commit.id());
                                }
                            }
                        }
                    }
                }
            } else {
                println!("Failed to retrieve repository status.");
            }
        }
        Err(e) => {
            println!("Failed to open repository: {}", e);
        }
    }
}
