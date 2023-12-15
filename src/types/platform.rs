use std::fmt::Display;
use anyhow::Error;

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
    write!(f, "{}", match self
    {
      Self::WindowsX32 => "windows-x32",
      Self::WindowsX64 => "windows-x64",
      Self::LinuxX32 => "linux-x32",
      Self::LinuxX64 => "linux-x64",
      Self::Android => "android",
      _ => "unknown"
    })
  }
}

impl Default for PlatformArch
{
  fn default() -> Self
  {
    Self::Unknown
  }
}

impl PlatformArch
{
  pub fn from_env() -> Result<Self, Error>
  {
    let os = whoami::platform();
    let arch = whoami::arch().width()?;
    Ok(match (os, arch)
    {
      (whoami::Platform::Windows, whoami::Width::Bits64) => Self::WindowsX64,
      (whoami::Platform::Windows, whoami::Width::Bits32) => Self::WindowsX32,
      (whoami::Platform::Linux, whoami::Width::Bits64) => Self::LinuxX64,
      (whoami::Platform::Linux, whoami::Width::Bits32) => Self::LinuxX32,
      (whoami::Platform::Android, _) => Self::Android,
      _ => Self::Unknown
    })
  }
}