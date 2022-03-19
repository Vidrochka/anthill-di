#![feature(unsize)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]

mod core_context;
pub (crate) use core_context::*;

mod dependency_context;
pub use dependency_context::*;

mod dependency_type;
pub use dependency_type::*;

mod life_cycle;
pub use life_cycle::*;

mod dependency;
pub (crate) use dependency::*;

mod dependency_link;
pub (crate) use dependency_link::*;

mod dependency_scope;
pub use dependency_scope::*;

mod dependency_builder;
pub use dependency_builder::*;

mod constructor;
pub use constructor::*;

mod constructors;
pub use constructors::*;

pub mod types;

pub mod extensions;

pub mod tests;