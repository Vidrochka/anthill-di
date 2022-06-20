#![feature(unsize)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]
#![feature(coerce_unsized)]
#![feature(trait_alias)]

mod core_context;
pub (crate) use core_context::*;

mod dependency_context;
pub use dependency_context::*;

mod life_cycle;
pub use life_cycle::*;

mod component;
pub (crate) use component::*;

#[cfg(feature = "loop-check")]
mod dependency_link;
#[cfg(feature = "loop-check")]
pub (crate) use dependency_link::*;

mod local_context;
pub use local_context::*;

mod service_mapping_builder;
pub use service_mapping_builder::*;

mod constructor;
pub use constructor::*;


mod global_context;
pub (crate) use global_context::*;

mod constructors;
pub (crate) use constructors::*;

pub mod types;
pub mod service;
pub mod cycled_components;

#[cfg(test)]
pub mod tests;