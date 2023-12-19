#[derive(clap::Parser)]
pub struct Args {
  /// Print rdm2 version
  #[arg(short, long)] pub version: bool,

  /// Load all dependencies for the current project from rdm2.toml file
  #[arg(short, long)] pub load: bool,

  /// Overrides default registry url: http://uav.radar-mms.com/gitlab/test/rdm/rdm-registry
  #[arg(long)] pub registry: Option<String>,

  /// Forces rdm to use local offline registry
  #[arg(long)] pub offline: bool,

  /// Specifies the path to local registry
  #[arg(long)] pub local: Option<String>,

  /// Specifies target system. Useful for cross-compiling
  #[arg(long)] pub operating_system: Option<String>,

  /// Specifies target architecture. Useful for cross-compiling
  #[arg(long)] pub architecture: Option<String>,

  /// Creates empty project with given name in current directory
  #[arg(short, long)] pub init: Option<String>,

  /// Specifies build system as CMake
  #[arg(long)] pub cmake: bool,

  /// Specifies build system as Cargo
  #[arg(long)] pub cargo: bool
}