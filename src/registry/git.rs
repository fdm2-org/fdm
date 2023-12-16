use anyhow::Error;
use colored::Colorize;
use git2_credentials::CredentialHandler;
use crate::{log, warn};

pub fn clone_repo(url: &str, target_path: &str) -> Result<(), Error>
{
  log!("cloning git repository from {} to {}", url, target_path);
  let url = if !url.ends_with(".git") {
    warn!("fixing url: {}{}", url, ".git".red().bold());
    format!("{}.git", url)
  } else {
    url.to_string()
  };
  let mut cb = git2::RemoteCallbacks::new();
  let git_config = git2::Config::open_default().unwrap();
  let mut ch = CredentialHandler::new(git_config);
  cb.credentials(move |url, username, allowed|
    ch.try_next_credential(url, username, allowed)
  );
  let mut fo = git2::FetchOptions::new();
  fo.remote_callbacks(cb)
    .download_tags(git2::AutotagOption::All)
    .update_fetchhead(true);
  std::fs::create_dir_all(&target_path)?;
  git2::build::RepoBuilder::new()
    .branch("main")
    .fetch_options(fo)
    .clone(url.as_str(), target_path.as_ref())?;
  Ok(())
}

pub fn pull_repo_main(url: &str, target_path: &str) -> Result<(), Error>
{
  log!("removing old git repository at {}", target_path);
  std::fs::remove_dir_all(target_path)?;
  clone_repo(url, target_path)?;
  Ok(())
}
