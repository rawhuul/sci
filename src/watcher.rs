use git2::{Oid, Repository, StatusOptions};
use std::{fmt::Debug, write};

pub struct Watcher {
    repo: Repository,
    latest_commit: Oid,
}

impl Debug for Watcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Watch[repo: {:?}, latest_commit: {}]",
            self.repo.workdir().unwrap(),
            self.latest_commit
        )
    }
}

impl Watcher {
    pub fn new(repo: Repository) -> Self {
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

    pub fn watch(&mut self) -> bool {
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        status_opts.include_ignored(false);
        status_opts.exclude_submodules(false);

        let statuses = self.repo.statuses(Some(&mut status_opts)).unwrap();

        if !statuses.is_empty() {
            print!("File changed: [");
            for st in statuses.iter() {
                print!("{:?}, ", st.path().unwrap());
            }
            println!("]");

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
