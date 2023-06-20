use git2::{Oid, Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Serialize, Deserialize)]
enum ChangeType {
    NewCommit,
    FileChange,
    Both,
}

impl Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewCommit => write!(f, "New Commit"),
            Self::FileChange => write!(f, "File Changes"),
            Self::Both => write!(f, "File Changes and New Commits"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Changes {
    change_type: ChangeType,
    files: Vec<String>,
    commit_id: Option<String>,
}

impl Debug for Changes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Changes")
            .field("change_type", &self.change_type)
            .field("files", &self.files)
            .field("commit_id", &self.commit_id)
            .finish()
    }
}

impl Changes {
    fn new(change_type: ChangeType, files: Vec<String>, commit_id: Option<Oid>) -> Self {
        let commit_id = commit_id.map(|oid| oid.to_string());

        Self {
            change_type,
            files,
            commit_id,
        }
    }
}

pub struct Watcher {
    repo: Repository,
    latest_commit: Oid,
}

impl Debug for Watcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Watcher")
            .field("repo", &self.repo.workdir().unwrap())
            .field("latest_commit", &self.latest_commit)
            .finish()
    }
}

impl Watcher {
    pub fn new(repo: Repository) -> Result<Self, git2::Error> {
        let latest_commit = repo.head()?.peel_to_commit()?.id();

        Ok(Self {
            repo,
            latest_commit,
        })
    }

    pub fn watch(&mut self) -> Result<Option<Changes>, git2::Error> {
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        status_opts.include_ignored(false);
        status_opts.exclude_submodules(false);

        let statuses = self.repo.statuses(Some(&mut status_opts))?;

        if !statuses.is_empty() && self.get_latest_commit_id()? != self.latest_commit {
            let files: Vec<String> = statuses
                .iter()
                .map(|st| st.path().unwrap().to_string())
                .collect();
            self.latest_commit = self.get_latest_commit_id()?;
            Ok(Some(Changes::new(
                ChangeType::Both,
                files,
                Some(self.latest_commit),
            )))
        } else if !statuses.is_empty() {
            let files: Vec<String> = statuses
                .iter()
                .map(|st| st.path().unwrap().to_string())
                .collect();
            Ok(Some(Changes::new(ChangeType::FileChange, files, None)))
        } else if self.get_latest_commit_id()? != self.latest_commit {
            self.latest_commit = self.get_latest_commit_id()?;
            Ok(Some(Changes::new(
                ChangeType::NewCommit,
                Vec::new(),
                Some(self.latest_commit),
            )))
        } else {
            Ok(None)
        }
    }

    fn get_latest_commit_id(&self) -> Result<Oid, git2::Error> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.id())
    }
}
