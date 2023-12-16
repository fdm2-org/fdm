use std::fmt::Display;
use std::path::Path;
use anyhow::{ensure, Error};
use colored::Colorize;
use crate::config::wd;
use crate::consts::{RDM_CACHE_NAME, RDM_DIRECTORY_NAME};
use crate::{fatal_error, log};
use crate::types::{Distribution, PlatformArch, Version};

#[derive(Debug, Clone)]
pub struct Dependency
{
  pub version: Version,
  pub distribution: Distribution,
  pub arch: Option<PlatformArch>
}

impl Default for Dependency
{
  fn default() -> Self
  {
    Self
    {
      version: Version::default(),
      distribution: Distribution::default(),
      arch: None
    }
  }
}

impl Display for Dependency
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{} ({}/{})",
           self.version,
           self.distribution,
           self.arch.as_ref().unwrap_or(&PlatformArch::default()))
  }
}

impl Dependency
{
  pub async fn download_from_registry(&self, name: &str) -> Result<(), Error>
  {
    log!("downloading {} {}/{}/{}",
      name.to_string().bright_blue().bold(),
      self.version.to_string().bold(),
      self.distribution.to_string().white().bold(),
      self.arch.as_ref().unwrap_or(&PlatformArch::Any).to_string().white().italic()
    );
    let path = self.cache_path(name);
    if !Path::new(&path).exists() {
      self.create_directory(name)?;
    }
    let registry = crate::registry::REGISTRY
      .lock()
      .unwrap();
    let url = registry
      .get(name, &self)?;
    // todo: download
    Ok(())
  }

  fn create_directory(&self, name: &str) -> Result<(), Error>
  {
    log!("creating cache directory at: {}", self.cache_path(name));
    std::fs::create_dir_all(&self.cache_path(name))?;
    Ok(())
  }

  fn cache_path(&self, name: &str) -> String
  {
    let path = Path::new(&wd().unwrap_or(String::from("")))
      .join(RDM_DIRECTORY_NAME)
      .join(RDM_CACHE_NAME)
      .join(format!("{}_{}_{}_{}",
                   name,
                   self.distribution,
                   self.version,
                   self.arch.as_ref().unwrap_or(&PlatformArch::Any))
        .as_str()
      ).into_os_string()
      .into_string()
      .expect("failed to convert path");
    path
  }
}