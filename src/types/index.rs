use std::collections::HashMap;
use crate::types::dependencies::Dependency;
use crate::types::{Descriptor, Version};

pub struct RegistryIndex
{
  pub versions: HashMap<Version, RegistryIndexPair>
}

pub struct RegistryIndexPair
{
  pub distribution: Descriptor,
  pub dependencies: HashMap<String, Dependency>
}