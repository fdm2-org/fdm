use serde::Deserialize;
use crate::types::{Distribution, Version};
use crate::types::dependencies::Dependency;

#[derive(Debug, Deserialize)]
pub struct DependencyPT
{
  pub version: String,
  pub distribution: String,
}

impl TryFrom<DependencyPT> for Dependency
{
  type Error = anyhow::Error;

  fn try_from(value: DependencyPT) -> Result<Self, Self::Error>
  {
    Ok(Self {
      version: Version::try_from(value.version.as_str())?,
      distribution: Distribution::from(value.distribution.as_str()),
      arch: None
    })
  }
}