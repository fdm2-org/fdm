mod cli;
mod consts;
mod config;
mod rdm2;
mod types;
mod registry;

use clap::Parser;
use crate::config::CONFIG;

pub use colored::Colorize;

fn main()
{
  let args = cli::args::Args::parse();
  if args.version
  {
    cli::out::version();
    std::process::exit(0);
  }

  cli::out::greet();
  CONFIG
    .lock()
    .unwrap()
    .load_args(&args)
    .dump_to_cli();
  match args.load {
    true => rdm2::run()
      .map_err(|err| {
        fatal_error!("{}", err);
        std::process::exit(1);
      })
      .unwrap(),
    false => {
      fatal_error!("no subcommand specified. see --help for more information");
      std::process::exit(1);
    }
  }
}
