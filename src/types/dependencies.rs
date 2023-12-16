use std::fmt::Display;
use crate::types::{Distribution, Version};

#[derive(Debug, Clone)]
pub struct Dependency
{
  pub name: String,
  pub version: Version,
  pub distribution: Distribution
}

impl Default for Dependency
{
  fn default() -> Self
  {
    Self
    {
      name: String::from(""),
      version: Version::default(),
      distribution: Distribution::default()
    }
  }
}

impl Display for Dependency
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}: {} ({})", self.name, self.version, self.distribution)
  }
}