use serde::Deserialize;
use crate::types::{Distribution, Version};

#[derive(Debug, Deserialize)]
pub struct DependencyPT
{
  pub version: String,
  pub distribution: String,
}

#[derive(Debug, Clone)]
pub struct Dependency
{
  pub version: Version,
  pub distribution: Distribution
}

impl TryFrom<DependencyPT> for Dependency
{
  type Error = anyhow::Error;

  fn try_from(value: DependencyPT) -> Result<Self, Self::Error>
  {
    Ok(Self {
      version: Version::try_from(value.version.as_str())?,
      distribution: Distribution::from(value.distribution.as_str())
    })
  }
}