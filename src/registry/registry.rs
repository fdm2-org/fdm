use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use anyhow::{bail, Error};
use colored::Colorize;
use lazy_static::lazy_static;
use crate::config::{CONFIG, wd};
use crate::consts::{RDM_DIRECTORY_NAME, RDM_REGISTRY_NAME};
use crate::log;
use crate::types::RegistryIndex;

lazy_static!
{
  pub static ref REGISTRY: Mutex<Registry> = Mutex::new(Registry::new().unwrap());
}

pub struct Registry
{
  pub path: String,
  pub index: HashMap<String, RegistryIndex>
}

impl Default for Registry
{
  fn default() -> Self
  {
    Self
    {
      path: String::from(""),
      index: HashMap::new()
    }
  }
}

impl Registry
{
  pub fn new() -> Result<Self, Error>
  {
    let path = Path::new(wd()?.as_str())
      .join(RDM_DIRECTORY_NAME)
      .join(RDM_REGISTRY_NAME)
      .into_os_string()
      .into_string()
      .unwrap_or(String::from(""));
    Ok(Self
    {
      path,
      index: HashMap::new()
    })
  }

  pub fn init_registry(&self) -> Result<&Self, Error>
  {
    log!("initializing registry at: {}", self.path);

    match self.seek_registry() {
      Ok(_) => {
        log!("found existing registry");
        self.update_registry()?;
      },
      Err(e) => {
        log!("{}", e);
        log!("creating registry folder");
        std::fs::create_dir_all(self.path.as_str())?;
        self.clone_registry()?;
      }
    };
    log!("{}" , "registry initialized!".green().bold());
    Ok(self)
  }

  fn seek_registry(&self) -> Result<(), Error>
  {
    if !Path::new(self.path.as_str()).exists() {
      bail!("registry not found at: {}", self.path);
    }
    if std::fs::read_dir(self.path.as_str())?.next().is_none() {
      bail!("registry is empty at: {}", self.path);
    }
    Ok(())
  }

  // online only
  fn clone_registry(&self) -> Result<(), Error>
  {
    let cfg = CONFIG
      .lock()
      .unwrap();
    let url = cfg
      .online_registry_url
      .as_str();
    log!("cloning registry from: {}", url);
    crate::registry::git::clone_repo(url, self.path.as_str())?;
    Ok(())
  }

  // online only
  fn update_registry(&self) -> Result<(), Error>
  {
    let cfg = CONFIG
      .lock()
      .unwrap();
    let url = cfg
      .online_registry_url
      .as_str();
    log!("updating registry from: {}", url);
    crate::registry::git::pull_repo_main(url, self.path.as_str())?;
    Ok(())
  }
}