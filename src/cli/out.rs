use std::string::ToString;
use colored::Colorize;
use lazy_static::lazy_static;
use crate::consts::RDM_NAME;

lazy_static!
{
  pub static ref RDM_NAME_COLORED: String = RDM_NAME.cyan().bold().to_string();
  pub static ref LOGGING_WARNING_MSG: String = "warning".yellow().bold().to_string();
  pub static ref LOGGING_ERROR_MSG: String = "unrecoverable error".red().bold().to_string();
}

#[macro_export] macro_rules! log {
    ($($arg:tt)*) => {
        println!("{}: {}", crate::cli::out::RDM_NAME_COLORED.as_str(),
          format_args!($($arg)*)
        );
    };
}

#[macro_export] macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}: {}: {}", crate::cli::out::RDM_NAME_COLORED.as_str(),
          crate::cli::out::LOGGING_WARNING_MSG.as_str(),
          format_args!($($arg)*).to_string().yellow().bold()
        );
    };
}

#[macro_export] macro_rules! fatal_error {
    ($($arg:tt)*) => {
        println!("{}: {}: {}", crate::cli::out::RDM_NAME_COLORED.as_str(),
          crate::cli::out::LOGGING_ERROR_MSG.as_str(),
          format_args!($($arg)*).to_string().red().bold().underline()
        );
    };
}