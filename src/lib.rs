#![feature(type_name_of_val)]
#![feature(unsize)]
#![feature(async_closure)]

mod container;
pub use container::*;
mod injector;
pub use injector::*;
mod injection;
pub use injection::*;
mod error;
pub use error::*;
pub mod builders;

pub mod tests;