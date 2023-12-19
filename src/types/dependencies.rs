use futures_util::stream::StreamExt;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::{Path};
use anyhow::{ensure, Error};
use colored::Colorize;
use decompress::{ExtractOptsBuilder};
use indicatif::{ProgressBar, ProgressDrawTarget};
use crate::config::wd;
use crate::consts::{FDM_CACHE_NAME, FDM_DIRECTORY_NAME, FDM_LIBS_NAME, FDM_PACK_NAME};
use crate::{log};
use crate::types::{Distribution, PlatformArch, Version};

#[derive(Debug, Clone)]
pub struct Dependency
{
  pub version: Version,
  pub distribution: Distribution,
  pub arch: Option<PlatformArch>
}

impl Default for Dependency
{
  fn default() -> Self
  {
    Self
    {
      version: Version::default(),
      distribution: Distribution::default(),
      arch: None
    }
  }
}

impl Display for Dependency
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{} ({}/{})",
           self.version,
           self.distribution,
           self.arch.as_ref().unwrap_or(&PlatformArch::default()))
  }
}

impl Dependency
{
  pub async fn download_from_registry(&self, name: &str) -> Result<(), Error>
  {
    log!("downloading {} {}/{}/{}",
      name.to_string().bright_blue().bold(),
      self.version.to_string().bold(),
      self.distribution.to_string().white().bold(),
      self.arch.as_ref().unwrap_or(&PlatformArch::Any).to_string().white().italic()
    );
    let path = self.cache_path(name);
    if !Path::new(&path).exists() {
      self.create_directory(name)?;
    }
    let registry = crate::registry::REGISTRY
      .lock()
      .unwrap();
    let url = registry
      .get(name, &self)?;

    let response = registry.client
      .get(url.as_str())
      .send()
      .await?;

    ensure!(response.status().is_success(), format!("failed to download: status code {}", response.status().as_str()));
    let total = response
      .content_length()
      .unwrap_or(1);
    let pb = ProgressBar::new(total)
      .with_style(
        indicatif::ProgressStyle::default_bar()
          .template("{wide_msg} {bytes_per_sec:8} {elapsed_precise:8} eta:{eta:3} {spinner:.green} \
                    [{bar:20.cyan/blue}] \
                    {bytes:10}/ {total_bytes:10} ({percent:3}%)"
          )
          .expect("template should be valid")
          .progress_chars("█░░")
      );
    pb.set_draw_target(ProgressDrawTarget::stdout_with_hz(5));
    pb.set_message(format!("downloading {}...", name));

    let file_path = Path::new(&path)
      .join("archive.tar.gz");
    let mut file = File::create(&file_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
      let chunk = item?;
      file.write_all(&chunk)?;
      let new = (downloaded + chunk.len() as u64).min(total);
      downloaded = new;
      pb.set_position(new);
    }
    pb.finish_with_message("done!");
    let target = Path::new(&wd()?)
      .join(FDM_DIRECTORY_NAME)
      .join(FDM_PACK_NAME)
      .join(FDM_LIBS_NAME)
      .join(name)
      .into_os_string()
      .into_string()
      .expect("os string should be convertible to string");
    Self::unpack(&file_path.to_str().expect("file path should be convertible to string"), &target)?;
    Ok(())
  }

  fn create_directory(&self, name: &str) -> Result<(), Error>
  {
    let p = self.cache_path(name);
    log!("creating cache directory at: ...{}", &p[p.len()-30..]);
    std::fs::create_dir_all(&self.cache_path(name))?;
    Ok(())
  }

  fn unpack(from: &str, to: &str) -> Result<(), Error>
  {
    log!("unpacking...");
    std::fs::create_dir_all(to)?;
    let _file = std::fs::File::open(from)?;
    decompress::decompress(from, to, &ExtractOptsBuilder::default().strip(1).build()?)?;
    Ok(())
  }

  fn cache_path(&self, name: &str) -> String
  {
    let path = Path::new(&wd().unwrap_or(String::from("")))
      .join(FDM_DIRECTORY_NAME)
      .join(FDM_CACHE_NAME)
      .join(format!("{}_{}_{}_{}",
                   name,
                   self.distribution,
                   self.version,
                   self.arch.as_ref().unwrap_or(&PlatformArch::Any))
        .as_str()
      ).into_os_string()
      .into_string()
      .expect("failed to convert path");
    path
  }
}