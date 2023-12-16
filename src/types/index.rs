use std::collections::HashMap;
use url::Url;
use crate::types::dependencies::Dependency;
use crate::types::{Distribution, PlatformArch, Version};

pub struct RegistryIndex
{
  pub versions: HashMap<Version, Descriptor>
}

pub struct Descriptor
{
  pub distribution: HashMap<Distribution, HashMap<PlatformArch, Url>>,
  pub dependencies: HashMap<String, Dependency>
}

impl Default for RegistryIndex
{
  fn default() -> Self
  {
    Self
    {
      versions: HashMap::new()
    }
  }
}

impl Default for Descriptor
{
  fn default() -> Self
  {
    Self
    {
      distribution: HashMap::new(),
      dependencies: HashMap::new()
    }
  }
}