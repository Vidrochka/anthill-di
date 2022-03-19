use std::any::TypeId;

#[derive(Debug)]
pub (crate) struct DependencyLink {
    pub (crate) parents: Vec<TypeId>,
    pub (crate) childs: Vec<TypeId>,
}

impl DependencyLink {
    pub (crate) fn new() -> Self {
        Self { parents: Vec::new(), childs: Vec::new() }
    }
}