use std::{
    any::{
        Any,
        TypeId
    },
    collections::HashMap,
    sync::Arc
};

use tokio::sync::RwLock;

pub struct DependencyScope {
    pub (crate) scoped_dependencies: RwLock<HashMap<TypeId, Arc<RwLock<Option<Arc<dyn Any + Sync + Send>>>>>>,
}

impl DependencyScope {
    pub fn new() -> Self { Self { scoped_dependencies: RwLock::new( HashMap::new() ) } }
}