//! Module containing the code that interacts with the file system. Mainly used for IO.
mod csv;
mod ply;
mod prelude;

// Public facing functions
pub use self::prelude::{get_header, read, write};
