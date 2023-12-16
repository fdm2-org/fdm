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
use crate::types::{Distribution, PlatformArch, RegistryIndex, Descriptor, Version};

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
    let mut registry_index = RegistryIndex::default();
    for hash in yaml {
      let content = hash
        .as_hash()
        .context("hash is not hash")?;
      for (k, v) in content {
        let version = k.as_str().context("key is not string")?;
        let entry = v.clone().into_hash().context("value is not hash")?;
        let mut registry_index_pair = Descriptor::default();
        for (key, value) in entry {
          match key.as_str().context("key is not string")? {
            "dependencies" => {
              // TODO: parse dependencies
            },
            _ => {
              let distribution = Distribution::from(value
                .as_str()
                .context("value is not string")?
              );
              let mut descriptor = PlatformDescriptor
              {
                distribution,
                urls: HashMap::new()
              };
              match key.as_str().context("key is not string")? {
                "source" => {
                  let download_url = Url::parse(value.as_str().context("url is not string")?)?;
                  descriptor.urls.insert(PlatformArch::Any, download_url);
                  registry_index_pair.distribution = descriptor;
                },
                _ => {
                  let platforms = key.into_hash().context("platforms is not hash")?;
                  for (platform, url) in platforms {
                    let arch = PlatformArch::from(platform
                      .as_str()
                      .context("platform is not string")?
                    );
                    let download_url = Url::parse(url.as_str().context("url is not string")?)?;
                    descriptor.urls.insert(arch, download_url);
                  }
                  registry_index_pair.distribution = descriptor;
                }
              }
            }
          }
        }
        registry_index.versions.insert(Version::try_from(version)?, registry_index_pair);
      }
    }
    Ok(registry_index)
  }

  fn dump_to_cli(&self) -> Result<(), Error>
  {
    println!();
    log!("{}", "-- registry index --".cyan().bold());
    for (package, idx) in &self.index
    {
      let name = package;
      let mut str: String = String::new();
      // for (version, pair) in &idx.versions
      // {
      //   str.push_str(&format!(
      //     "{}: {} {:?}; ",
      //     version.to_string().yellow().bold(),
      //     &pair.distribution.to_string().bold(),
      //     &pair.distribution.urls.keys()
      //   ));
      // }

    }
    println!();
    Ok(())
  }
}