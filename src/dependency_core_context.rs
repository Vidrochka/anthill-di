use std::{
    collections::HashMap,
    any::{
        TypeId,Any
    },
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
    pub (crate) dependency_link_collection: RwLock<HashMap<TypeId, Arc<RwLock<DependencyLink>>>>,
    pub (crate) singleton_dependency: RwLock<HashMap<TypeId, Arc<RwLock<Option<Box<dyn Any>>>>>> // первые Arc<RwLock<>> для создания singletone без блокировки всей коллекции (блокируем отдельный тип)
}

impl DependencyCoreContext {
    pub (crate) fn new() -> Self {
        Self {
            dependency_collection: RwLock::new(HashMap::new()),
            dependency_link_collection: RwLock::new(HashMap::new()),
            singleton_dependency: RwLock::new(HashMap::new()),
        }
    }
}