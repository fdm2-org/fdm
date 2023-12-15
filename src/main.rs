mod cli;
mod consts;
use colored::Colorize;

fn main() {
    log!("{} {}", consts::RDM_NAME, consts::RDM_VERSION);
    warn!("{} {}", consts::RDM_NAME, consts::RDM_VERSION);
    fatal_error!("{} {}", consts::RDM_NAME, consts::RDM_VERSION);
}
