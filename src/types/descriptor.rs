use std::collections::HashMap;
use url::Url;
use crate::types::{Distribution, PlatformArch};

#[derive(Debug)]
pub struct Descriptor
{
  pub distribution: Distribution,
  pub urls: HashMap<PlatformArch, Url>
}

impl Default for Descriptor
{
  fn default() -> Self
  {
    Self
    {
      distribution: Distribution::Unknown,
      urls: HashMap::new()
    }
  }
}

