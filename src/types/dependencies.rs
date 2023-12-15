use std::fmt::Display;
use crate::types::{Distribution, PlatformArch, Version};

#[derive(Debug, Clone)]
pub struct Dependency
{
  pub name: String,
  pub version: Version,
  pub distribution: Distribution,
  pub platform: PlatformArch
}

impl Default for Dependency
{
  fn default() -> Self
  {
    Self
    {
      name: String::from(""),
      version: Version::default(),
      distribution: Distribution::default(),
      platform: PlatformArch::default()
    }
  }
}

impl Display for Dependency
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}: {} ({}/{})", self.name, self.version, self.platform, self.distribution)
  }
}