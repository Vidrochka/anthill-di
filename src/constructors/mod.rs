mod component_from_constructor;
pub (crate) use component_from_constructor::*;

#[cfg(feature = "async-mode")]
mod component_from_async_closure;
#[cfg(feature = "async-mode")]
pub (crate) use component_from_async_closure::*;

mod component_from_closure;
pub (crate) use component_from_closure::*;

mod component_from_instance;
pub (crate) use component_from_instance::*;