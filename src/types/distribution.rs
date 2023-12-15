use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Distribution
{
  Static,
  Shared,
  Sources,
  Unknown
}

impl Default for Distribution
{
  fn default() -> Self
  {
    Self::Unknown
  }
}

impl From<&str> for Distribution
{
  fn from(value: &str) -> Self
  {
    match value
    {
      "static" => Self::Static,
      "shared" => Self::Shared,
      "sources" | "src" | "source" => Self::Sources,
      _ => Self::Unknown
    }
  }
}

impl From<String> for Distribution
{
  fn from(value: String) -> Self
  {
    Self::from(value.as_str())
  }
}

impl Display for Distribution
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", self.to_string())
  }
}