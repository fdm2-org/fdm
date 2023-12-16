use serde::Deserialize;
use crate::types::Version;

#[derive(Debug, Clone, Deserialize)]
pub struct PackagePT
{
  pub name: String,
  pub version: String,
  pub authors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Package
{
  pub name: String,
  pub version: Version,
  pub authors: Vec<String>,
}

impl Default for PackagePT
{
  fn default() -> Self
  {
    Self
    {
      name: String::from(""),
      version: String::from(""),
      authors: Vec::new()
    }
  }
}

impl Default for Package
{
  fn default() -> Self
  {
    Self
    {
      name: String::from(""),
      version: Version::default(),
      authors: Vec::new()
    }
  }
}

impl TryFrom<PackagePT> for Package
{
  type Error = anyhow::Error;

  fn try_from(value: PackagePT) -> Result<Self, Self::Error>
  {
    Ok(Self
    {
      name: value.name,
      version: Version::try_from(value.version.as_str())?,
      authors: value.authors
    })
  }
}