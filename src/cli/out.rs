use std::string::ToString;
use lazy_static::lazy_static;
use crate::consts::{FDM_NAME, FDM_VERSION};

pub use colored::Colorize;

lazy_static!
{
  pub static ref FDM_NAME_COLORED: String = FDM_NAME.cyan().bold().to_string();
  pub static ref LOGGING_WARNING_MSG: String = "⚠ warning".yellow().bold().to_string();
  pub static ref LOGGING_ERROR_MSG: String = "⛔ unrecoverable error occurred".red().bold()
  .to_string();
}

#[macro_export] macro_rules! log {
    ($($arg:tt)*) => {
        println!("{}: {}", crate::cli::out::FDM_NAME_COLORED.as_str(),
          format_args!($($arg)*)
        );
    };
}

#[macro_export] macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}: {}: {}", crate::cli::out::FDM_NAME_COLORED.as_str(),
          crate::cli::out::LOGGING_WARNING_MSG.as_str(),
          format_args!($($arg)*).to_string().yellow().bold()
        );
    };
}

#[macro_export] macro_rules! fatal_error {
    ($($arg:tt)*) => {
        println!("{}: {}: {}", crate::cli::out::FDM_NAME_COLORED.as_str(),
          crate::cli::out::LOGGING_ERROR_MSG.as_str(),
          format_args!($($arg)*).to_string().red().bold().underline()
        );
    };
}

pub fn greet()
{
  println!("starting {} version {}",
           FDM_NAME.yellow().bold(),
           FDM_VERSION.magenta().bold()
  );
  println!()
}

pub fn version()
{
  println!("{} version {}",
           "free dependency manager".cyan().bold(),
           FDM_VERSION.magenta().bold()
  );
  println!("built from branch: {}",
           option_env!("GIT_BRANCH").unwrap_or("unknown").bold()
  );
  println!("commit: {}",
           option_env!("GIT_COMMIT").unwrap_or("unknown").bold()
  );
  println!("dirty: {}",
           option_env!("GIT_DIRTY").unwrap_or("unknown").bold()
  );
  println!("build timestamp: {}",
           option_env!("SOURCE_TIMESTAMP").unwrap_or("unknown").green().bold()
  );
  println!("{}", "2023 whs31 (c) no rights reserved".white());
}