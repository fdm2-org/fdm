use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum PlatformArch
{
  WindowsX32,
  WindowsX64,
  LinuxX32,
  LinuxX64,
  Android,
  Unknown
}

impl From<&str> for PlatformArch
{
  fn from(s: &str) -> Self
  {
    match s
    {
      "windows-x32" => Self::WindowsX32,
      "windows-x64" => Self::WindowsX64,
      "linux-x32" => Self::LinuxX32,
      "linux-x64" => Self::LinuxX64,
      "android" => Self::Android,
      _ => Self::Unknown
    }
  }
}

impl From<String> for PlatformArch
{
  fn from(s: String) -> Self
  {
    Self::from(s.as_str())
  }
}

impl Display for PlatformArch
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", self.to_string())
  }
}

impl Default for PlatformArch
{
  fn default() -> Self
  {
    Self::Unknown
  }
}