use std::fs::File;
use std::io::Write;
use anyhow::Error;
use crate::cmake::CMakeCommand;

pub struct CMakeFile
{
  pub path: String,
  pub content: String
}

impl CMakeFile
{
  pub fn from_file(path: &str) -> Result<Self, Error>
  {
    Ok(Self
    {
      path: path.to_string(),
      content: std::fs::read_to_string(&path)?
    })
  }

  pub fn new(path: &str) -> Result<Self, Error>
  {
    if std::path::Path::new(&path).exists() {
      std::fs::remove_file(&path)?;
    }
    Ok(Self
    {
      path: path.to_string(),
      content: String::new()
    })
  }

  pub fn commit(&self) -> Result<(), Error>
  {
    let mut file = File::create(&self.path)?;
    file.write_all(self.content.as_bytes())?;
    Ok(())
  }

  pub fn append(&mut self, content: &str) -> Result<&mut Self, Error>
  {
    self.content.push_str(content);
    self.content.push_str("\n");
    Ok(self)
  }

  pub fn command(&mut self, cmd: CMakeCommand) -> Result<&mut Self, Error>
  {
    self.content.push_str(&format!("{}\n", cmd));
    Ok(self)
  }
}

