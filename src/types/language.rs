use anyhow::bail;

#[derive(Debug)]
pub enum Language
{
  C,
  CPP,
  CSharp,
  QML,
  JavaScript,
  Python,
  Rust,
  Go,
  Undefined
}

impl Default for Language
{
  fn default() -> Self
  {
    Self::Undefined
  }
}

impl TryFrom<&str> for Language
{
  type Error = anyhow::Error;

  fn try_from(s: &str) -> Result<Self, Self::Error>
  {
    match s.to_lowercase().as_str()
    {
      "c" => Ok(Self::C),
      "cpp" | "c++" | "cxx" => Ok(Self::CPP),
      "c#" | "csharp" => Ok(Self::CSharp),
      "qml" => Ok(Self::QML),
      "js" | "javascript" => Ok(Self::JavaScript),
      "py" | "python" => Ok(Self::Python),
      "rs" | "rust" => Ok(Self::Rust),
      "go" | "golang" => Ok(Self::Go),
      _ => bail!("unknown language: {}", s)
    }
  }
}

impl TryFrom<String> for Language
{
  type Error = anyhow::Error;

  fn try_from(s: String) -> Result<Self, Self::Error>
  {
    Self::try_from(s.as_str())
  }
}

impl std::fmt::Display for Language
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      Self::C => write!(f, "C"),
      Self::CPP => write!(f, "C++"),
      Self::CSharp => write!(f, "C#"),
      Self::QML => write!(f, "QML"),
      Self::JavaScript => write!(f, "JavaScript"),
      Self::Python => write!(f, "Python"),
      Self::Rust => write!(f, "Rust"),
      Self::Go => write!(f, "Go"),
      Self::Undefined => write!(f, "Undefined")
    }
  }
}