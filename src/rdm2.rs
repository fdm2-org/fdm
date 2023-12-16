use anyhow::Error;
use crate::registry::REGISTRY;

pub fn run() -> Result<(), Error>
{
  REGISTRY
    .lock()
    .unwrap()
    .init_registry()?;
  Ok(())
}