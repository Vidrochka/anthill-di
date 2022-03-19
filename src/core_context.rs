use std::{
    collections::HashMap,
    any::TypeId,
    sync::Arc
};

use tokio::sync::RwLock;

use crate::{
    Dependency,
    DependencyLink
};

#[derive(Debug)]
pub (crate) struct DependencyCoreContext {
    pub (crate) dependency_collection: RwLock<HashMap<TypeId, Arc<Dependency>>>,
    pub (crate) dependency_link_collection: RwLock<HashMap<TypeId, DependencyLink>>,
}

impl DependencyCoreContext {
    pub (crate) fn new() -> Self {
        Self {
            dependency_collection: RwLock::new(HashMap::new()),
            dependency_link_collection: RwLock::new(HashMap::new()),
        }
    }
}