#![feature(type_name_of_val)]
#![feature(unsize)]

mod container;
pub use container::*;
mod injector;
pub use injector::*;
mod injection;
pub use injection::*;
pub mod builders;

pub mod tests;