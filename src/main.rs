use git2::{Oid, Repository};
use std::{env, fmt::Debug, format, path::PathBuf, thread, time, write};

struct Watch {
    repo: Repository,
    latest_commit: Oid,
}

impl Debug for Watch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Watch[repo: {:?}, latest_commit: {}]",
            self.repo.workdir().unwrap(),
            self.latest_commit
        )
    }
}

impl Watch {
    fn new(repo: Repository) -> Self {
        let latest_commit = repo
            .head()
            .expect("Failed to get HEAD reference")
            .peel_to_commit()
            .expect("Failed to peel HEAD to commit")
            .id();

        Self {
            repo,
            latest_commit,
        }
    }

    fn has_changed(&mut self) -> bool {
        let statuses = self.repo.statuses(None).unwrap();

        if !statuses.is_empty() {
            true
        } else if self.get_latest_commit_id() != self.latest_commit {
            self.latest_commit = self.get_latest_commit_id();
            true
        } else {
            false
        }
    }

    fn get_latest_commit_id(&self) -> Oid {
        let head = self.repo.head().unwrap();
        let commit = head.peel_to_commit().unwrap();
        commit.id()
    }
}

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

fn main() {
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

    let mut watch_dog = Watch::new(repo.unwrap());

    loop {
        let changed = watch_dog.has_changed();
        println!("{:?}", watch_dog);
        println!("{changed}");

        thread::sleep(time::Duration::from_secs(5));
    }
}
