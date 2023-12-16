pub mod platform;
pub mod build_system;
pub mod distribution;
pub mod version;
pub mod language;
pub mod dependencies;
pub mod index;

pub use platform::PlatformArch;
pub use build_system::BuildSystem;
pub use distribution::Distribution;
pub use version::Version;
pub use language::Language;
pub use index::
{
  RegistryIndex,
  Descriptor
};