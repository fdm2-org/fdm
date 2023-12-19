use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum CMakeCommand
{
  add_subdirectory(String),
  macro_start(String, String),
  macro_end,
  file_glob(FileGlobMode, String, FileRelativeMode, String, String),
  set(String, String),
  foreach_start(String, String),
  foreach_end,
  if_start(String),
  if_end,
  list(ListMode, String, String),
  any(String),
}

#[derive(Debug, Clone)]
pub enum FileGlobMode
{
  Glob,
  GlobRecurse
}

#[derive(Debug, Default, Clone)]
pub enum FileRelativeMode
{
  #[default] Default,
  Relative
}

#[derive(Debug, Clone)]
pub enum ListMode
{
  Append
}

impl Display for FileGlobMode
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      FileGlobMode::Glob => write!(f, "GLOB"),
      FileGlobMode::GlobRecurse => write!(f, "GLOB_RECURSE"),
    }
  }
}

impl Display for ListMode
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      ListMode::Append => write!(f, "APPEND")
    }
  }
}

impl Display for FileRelativeMode
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      FileRelativeMode::Default => write!(f, ""),
      FileRelativeMode::Relative => write!(f, "RELATIVE")
    }
  }
}

impl Display for CMakeCommand
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      CMakeCommand::add_subdirectory(path) => write!(f, "add_subdirectory({})", path),
      CMakeCommand::macro_start(name, args) => write!(f, "macro({} {})", name, args),
      CMakeCommand::macro_end => write!(f, "endmacro()"),
      CMakeCommand::file_glob(mode, path, relative, glob, body)
        => write!(f, "file({} {} {} {} {})", mode, path, relative, glob, body),
      CMakeCommand::set(name, value) => write!(f, "set({} {})", name, value),
      CMakeCommand::foreach_start(name, value) => write!(f, "foreach({} {})", name, value),
      CMakeCommand::foreach_end => write!(f, "endforeach()"),
      CMakeCommand::if_start(cond) => write!(f, "if({})", cond),
      CMakeCommand::if_end => write!(f, "endif()"),
      CMakeCommand::list(mode, name, value) => write!(f, "list({} {} {})", mode, name, value),
      CMakeCommand::any(cmd) => write!(f, "{}", cmd)
    }
  }
}