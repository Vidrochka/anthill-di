use std::any::TypeId;
use derive_new::new;

#[derive(Debug, new)]
pub (crate) struct DependencyLink {
    #[new(default)] pub (crate) parents: Vec<TypeId>,
    #[new(default)] pub (crate) childs: Vec<TypeId>,
}