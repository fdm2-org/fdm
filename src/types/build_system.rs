use std::fmt::Display;

#[derive(Debug)]
pub enum BuildSystem
{
  CMake,
  Cargo,
  Unknown
}

impl Default for BuildSystem
{
  fn default() -> Self
  {
    Self::Unknown
  }
}

impl From<&str> for BuildSystem
{
  fn from(s: &str) -> Self
  {
    match s
    {
      "cmake" => Self::CMake,
      "cargo" => Self::Cargo,
      _ => Self::Unknown
    }
  }
}

impl From<String> for BuildSystem
{
  fn from(s: String) -> Self
  {
    Self::from(s.as_str())
  }
}

impl Display for BuildSystem
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", self.to_string())
  }
}