use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use anyhow::{bail, Context, Error};
use colored::Colorize;
use lazy_static::lazy_static;
use url::Url;
use walkdir::WalkDir;
use yaml_rust::Yaml;
use crate::config::{CONFIG, wd};
use crate::consts::{RDM_DIRECTORY_NAME, RDM_REGISTRY_NAME};
use crate::log;
use crate::types::{
  Distribution,
  PlatformArch,
  RegistryIndex,
  Descriptor,
  Version
};
use crate::types::dependencies::Dependency;

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

  pub fn init_registry(&mut self) -> Result<&Self, Error>
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
    for yaml in self.collect_yaml()? {
      self.index.insert(yaml.0, self.parse_yaml(&yaml.1)?);
    }

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

  fn collect_yaml(&self) -> Result<Vec<(String, Vec<Yaml>)>, Error>
  {
    log!("collecting yaml files from registry");
    let mut yaml_files = vec![];
    for entry in WalkDir::new(self.path.as_str())
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().is_file()
        && e.path().extension().is_some()
        && e.path().extension().unwrap() == "yml"
      )
    {
      let content = std::fs::read_to_string(entry.path())?;
      let yaml = yaml_rust::YamlLoader::load_from_str(content.as_str())?;
      yaml_files.push((
        entry
          .path()
          .file_stem()
          .context("file stem is none")?
          .to_os_string()
          .into_string()
          .expect("os string to string should not fail"),
        yaml
      ));
    }
    log!("found {} yaml files", yaml_files.len());
    Ok(yaml_files)
  }

  fn parse_yaml(&self, yaml: &Vec<Yaml>) -> Result<RegistryIndex, Error>
  {
    let mut index = RegistryIndex::default();
    for hash in yaml
    {
      let content = hash
        .as_hash()
        .context("hash is none")?;
      for (version, descriptor) in content
      {
        let version = Version::try_from(version
          .as_str()
          .context("version is none")?)?;
        let descriptor_hash = descriptor
          .as_hash()
          .context("descriptor is none")?;
        //let mut distribution = HashMap::new();
        let mut dependencies = HashMap::new();
        for (key, value) in descriptor_hash
        {
          match key.as_str().context("key is none")? {
            "dependencies" => {
              let value = value
                .as_vec()
                .context("value is none (array)")?;
              for hash in value
              {
                let hash = hash
                  .as_hash()
                  .context("value is none (hash)")?;
                for (name, dependency) in hash
                {
                  let name = name
                    .as_str()
                    .context("name is none")?;
                  let dependency = dependency
                    .as_hash()
                    .context("dependency is none")?;
                  let mut dependency_struct = Dependency::default();
                  for (key, value) in dependency
                  {
                    let key = key
                      .as_str()
                      .context("key is none")?;
                    let value = value
                      .as_str()
                      .context("value is none")?;
                    match key {
                      "version" => dependency_struct.version = Version::try_from(value)?,
                      "distribution" => dependency_struct.distribution = Distribution::try_from(value)?,
                      _ => bail!("unknown key: {}", key)
                    }
                  }
                  dependencies.insert(name.to_string(), dependency_struct);
                }
              }
            },
            "source" => {
              let value = value
                .as_str()
                .context("value is none (sources)")?;
            }
            _ => {
              let value = value
                .as_hash()
                .context("value is none (other)")?;
            }
          }
        }
      }
    }
    Ok(index)
  }

  fn dump_to_cli(&self) -> Result<(), Error>
  {
    println!();
    log!("{}", "-- registry index --".cyan().bold());
    todo!();
    println!();
    Ok(())
  }
}