use std::path::Path;
use anyhow::{Context, Error};
use crate::config::wd;
use crate::types::BuildSystem;

macro_rules! copy_file {
    ($filename:literal) => {
      let content = include_bytes!($filename);
      let path = Path::new(&wd()?)
        .join($filename)
        .into_os_string()
        .into_string()
        .expect("failed to convert path");
      std::fs::write(&path, content)?;
    }
  }

pub struct Initializer
{
  pub name: String,
  pub build_system: BuildSystem
}

impl Initializer
{
  pub fn from_args(args: &crate::cli::args::Args) -> Result<Self, Error>
  {
    let bs = if args.cmake {
      BuildSystem::CMake
    } else if args.cargo {
      BuildSystem::Cargo
    } else {
      BuildSystem::Unknown
    };
    Ok(Self
    {
      name: args.init.as_ref().context("no name specified")?.to_string(),
      build_system: bs
    })
  }

  pub fn create(&self) -> Result<(), Error>
  {
    copy_file!(".clang-format");
    //copy_file!(".clang-tidy");
    copy_file!(".gitignore");
    copy_file!(".gitattributes");
    Ok(())
  }
}