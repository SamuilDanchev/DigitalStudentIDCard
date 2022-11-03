///The library can be extended in its function,
/// for example the headers can be encrypted with other encryption algorithms.
/// The error messages can be extended if necessary, these cover the most basic cases
pub use lib_error::LoginError;
pub use time::Expiration;
pub use token::{create_token, verify_token};
pub use user::{signin, LoginUser, User};

mod lib_error;
mod time;
mod token;
mod user;
