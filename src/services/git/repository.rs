use std::env;
use git2::Repository;
use log::{warn, info};

pub struct GitRepositoryService;

impl GitRepositoryService {
    pub fn resolve(repo: &str) -> Option<Repository> {
        let dir = env::var("GIT_DIR").expect("GIT_DIR is not defined");
        let path = format!("{}/{}", dir, repo);

        info!("Opening '{}' as git repository ...", repo);

        return match Repository::open(path.clone()) {
            Ok(git_repo) => Some(git_repo),
            _ => {
                warn!("Failed open '{}' as git repository", path);
                None
            },
        }
    }
}
