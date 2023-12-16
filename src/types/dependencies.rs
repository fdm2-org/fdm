use std::fmt::Display;
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