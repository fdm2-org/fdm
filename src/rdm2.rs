use anyhow::Error;
use crate::registry::REGISTRY;

pub fn run() -> Result<(), Error>
{
  REGISTRY
    .lock()
    .unwrap()
    .init_registry()?
    .dump_to_cli()?;
  let manifest = crate::manifest::Manifest::seek()?
    .dump_to_cli()?
    .download_dependencies()?;
  Ok(())
}