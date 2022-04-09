#![feature(unsize)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]
#![feature(coerce_unsized)]

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

mod dependency_constructor;
pub use dependency_constructor::*;

pub mod service;

mod cycled_component_builder;
pub (crate) use cycled_component_builder::*;

mod global_scope;
pub (crate) use global_scope::*;

mod constructors;
pub (crate) use constructors::*;

pub mod types;


#[cfg(test)]
pub mod tests;