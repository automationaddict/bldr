// custom error type for the pull_git_repo() function
pub enum GitError {
    GitNotInstalled,
    GitPullError,
}

pub fn pull_git_repo() -> Result<(), GitError> {
    let git_path = which("git").map_err(|_| GitError::GitNotInstalled)?;
    let git_pull = Command::new(git_path)
        .arg("pull")
        .output()
        .map_err(|_| GitError::GitPullError)?;
    if git_pull.status.success() {
        Ok(())
    } else {
        Err(GitError::GitPullError)
    }
}
