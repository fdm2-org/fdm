use std::collections::HashMap;
use std::path::Path;
use anyhow::{ensure, Error};
use colored::Colorize;
use serde::Deserialize;
use crate::config::wd;
use crate::consts::RDM_MANIFEST_FILENAME;
use crate::log;
use crate::manifest::{
  Dependency,
  DependencyPT,
  Package,
  PackagePT
};

#[derive(Debug)]
pub struct Manifest
{
  pub package: Package,
  pub dependencies: Option<HashMap<String, Dependency>>
}

#[derive(Debug, Deserialize)]
pub struct ManifestPT
{
  pub package: PackagePT,
  pub dependencies: Option<HashMap<String, DependencyPT>>
}

impl Default for Manifest
{
  fn default() -> Self
  {
    Self
    {
      package: Package::default(),
      dependencies: None
    }
  }
}

impl TryFrom<ManifestPT> for Manifest
{
  type Error = anyhow::Error;

  fn try_from(value: ManifestPT) -> Result<Self, Self::Error>
  {
    Ok(Self
    {
      package: Package::try_from(value.package)?,
      dependencies: value.dependencies
        .map(|deps| deps
          .into_iter()
          .map(|(name, dep)| {
            Ok((name, Dependency::try_from(dep)?))
          })
          .collect::<Result<HashMap<String, Dependency>, Error>>()
        )
        .transpose()?
    })
  }
}

impl Manifest
{
  pub fn from_toml_string(toml: &str) -> Result<ManifestPT, Error>
  {
    let manifest: ManifestPT = toml::from_str(toml)?;
    Ok(manifest)
  }

  pub fn from_toml_file(path: &str) -> Result<ManifestPT, Error>
  {
    let toml = std::fs::read_to_string(path)?;
    Self::from_toml_string(&toml)
  }

  pub fn seek() -> Result<Self, Error>
  {
    let path = Path::new(&wd()?)
      .join(RDM_MANIFEST_FILENAME)
      .into_os_string()
      .into_string()
      .expect("os string should be convertible to string");
    ensure!(Path::new(&path).exists(), "manifest not found in current directory");
    Self::try_from(
      Self::from_toml_file(path.as_str())?
    )
  }

  pub fn dump_to_cli(&self) -> Result<(), Error>
  {
    println!();
    log!("{}" , "-- manifest --".green().bold());
    log!("package {} version {} by {}",
      self.package.name.to_string().magenta().bold().underline(),
      self.package.version.to_string().yellow().bold(),
      self.package.authors.join(", ")
    );
    if let Some(deps) = &self.dependencies {
      log!("{} {}",
        self.package.name.to_string().magenta().bold(),
        "dependencies:".bold()
      );
      for (name, dep) in deps {
        log!("\t🔶 {} version {}/{}",
          name.to_string().cyan().bold(),
          dep.version.to_string().bold(),
          dep.distribution.to_string().white().bold()
        );
      }
    }
    Ok(())
  }
}