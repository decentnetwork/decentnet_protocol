mod utils;

#[cfg(feature = "builders")]
pub mod builders;
#[cfg(feature = "interface")]
pub mod interface;
#[cfg(feature = "templates")]
pub mod templates;

pub enum Error {
    Err(String),
}
