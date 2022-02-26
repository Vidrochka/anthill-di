#![feature(unsize)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]

mod dependency_core_context;
pub (crate) use dependency_core_context::*;

mod dependency_context;
pub use dependency_context::*;

mod dependency_type;
pub use dependency_type::*;

mod dependency_life_cycle;
pub use dependency_life_cycle::*;

mod dependency;
pub (crate) use dependency::*;

mod dependency_link;
pub (crate) use dependency_link::*;

mod dependency_scope;
pub use dependency_scope::*;

mod dependency_builder;
pub use dependency_builder::*;

mod dependency_constructor;
pub use dependency_constructor::*;

mod constructors;
pub use constructors::*;

pub mod types;

pub mod extensions;

pub mod tests;