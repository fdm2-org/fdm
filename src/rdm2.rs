use std::path::Path;
use anyhow::Error;
use crate::cmake::{CMakeFile, FileGlobMode, FileRelativeMode, ListMode};
use crate::config::wd;
use crate::consts::{RDM_DIRECTORY_NAME, RDM_LIBS_NAME, RDM_PACK_NAME};
use crate::registry::REGISTRY;
use crate::cmake::CMakeCommand::*;
use crate::log;

pub fn run() -> Result<(), Error>
{
  REGISTRY
    .lock()
    .unwrap()
    .init_registry()?
    .dump_to_cli()?;
  let _downloaded_names = crate::manifest::Manifest::seek()?
    .dump_to_cli()?
    .download_dependencies()?;
  let cmake_path_root = Path::new(&wd()?)
    .join(RDM_DIRECTORY_NAME)
    .join("CMakeLists.txt")
    .into_os_string()
    .into_string()
    .expect("os string should be convertible to string");
  let cmake_internal_path = Path::new(&wd()?)
    .join(RDM_DIRECTORY_NAME)
    .join(RDM_PACK_NAME)
    .join("CMakeLists.txt")
    .into_os_string()
    .into_string()
    .expect("os string should be convertible to string");
  let cmake_path = Path::new(&wd()?)
    .join(RDM_DIRECTORY_NAME)
    .join(RDM_PACK_NAME)
    .join(RDM_LIBS_NAME)
    .join("CMakeLists.txt")
    .into_os_string()
    .into_string()
    .expect("os string should be convertible to string");
  println!();
  log!("creating new root cmake file...");
  CMakeFile::new(&cmake_path_root)?
    .command(add_subdirectory(RDM_PACK_NAME.to_string()))?
    .commit()?;
  log!("creating new root cmake file.....\tOK");
  log!("creating internal cmake...");
  CMakeFile::new(&cmake_internal_path)?
    .command(add_subdirectory(RDM_LIBS_NAME.to_string()))?
    .commit()?;
  log!("creating internal cmake..........\tOK");
  log!("creating cmake collection file...");
  CMakeFile::new(&cmake_path)?
    .command(macro_start("SUBDIRLIST".to_string(), "result curdir".to_string()))?
    .command(file_glob(
      FileGlobMode::Glob,
      "children".to_string(),
      FileRelativeMode::Relative,
      "${curdir}".to_string(),
      "${curdir}/*".to_string()
    ))?
    .command(set("dirlist".to_string(), r#""""#.to_string()))?
    .command(foreach_start("child".to_string(), "${children}".to_string()))?
    .command(if_start("IS_DIRECTORY ${curdir}/${child}".to_string()))?
    .command(list(
      ListMode::Append,
      "dirlist".to_string(),
      "${child}".to_string()
    ))?
    .command(if_end)?
    .command(foreach_end)?
    .command(set("${result}".to_string(), "${dirlist}".to_string()))?
    .command(macro_end)?
    .command(any("\n".to_string()))?
    .command(any("SUBDIRLIST(SUBDIRS ${CMAKE_CURRENT_SOURCE_DIR})".to_string()))?
    .command(foreach_start(
      "subdir".to_string(),
      "${SUBDIRS}".to_string()
    ))?
    .command(add_subdirectory("${subdir}".to_string()))?
    .command(foreach_end)?
    .command(any("\n".to_string()))?
    .commit()?;
  log!("creating cmake collection file...\tOK");
  Ok(())
}