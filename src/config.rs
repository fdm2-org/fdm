use std::sync::Mutex;
use anyhow::Error;
use colored::Colorize;
use lazy_static::lazy_static;
use crate::{fatal_error, log, warn};

lazy_static!
{
  pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

#[derive(Debug)]
pub struct Config
{
  pub offline_registry_url: Option<String>,
  pub online_registry_url: String
}

impl Default for Config
{
  fn default() -> Self
  {
    Self
    {
      offline_registry_url: None,
      online_registry_url: String::from("http://uav.radar-mms.com/gitlab/test/rdm/rdm-registry")
    }
  }
}

impl Config
{
  pub fn dump_to_cli(&self)
  {
    println!();
    log!("{}", "-- configuration --".green().bold());
    if self.offline_registry_url.is_some()
    {
      warn!("using offline registry!");
      log!("offline registry url: {}", self.offline_registry_url.as_ref().unwrap());
    }
    else
    {
      log!("using online registry by default");
      log!("online registry url: {}", self.online_registry_url);
    }
    println!();
  }

  pub fn load_args(&mut self, args: &crate::cli::args::Args) -> &Self
  {
    if args.offline {
      if args.local.is_some() {
        self.offline_registry_url = Some(args.local.as_ref().unwrap().to_string());
      }
      else {
        fatal_error!("argument --offline requires --local to be set!");
        std::process::exit(1);
      }
    }
    if args.registry.is_some() {
      self.online_registry_url = args.registry.as_ref().unwrap().to_string();
    }
    self
  }
}

pub fn wd() -> Result<String, Error>
{
  Ok(std::env::current_dir()?
    .into_os_string()
    .into_string()
    .expect("os string should be convertible to string"))
}