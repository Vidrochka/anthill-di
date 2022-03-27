use std::fmt::Debug;
use crate::{ServiceMappingsCollection, ICycledComponentBuilder, GlobalScope};
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

//#[derive(Debug)]
pub (crate) struct DependencyCoreContext {
    pub (crate) components: RwLock<HashMap<TypeId, Arc<Dependency>>>,
    pub (crate) cycled_component_builders: RwLock<HashMap<TypeId, Arc<Box<dyn ICycledComponentBuilder>>>>,
    pub (crate) services: RwLock<HashMap<TypeId, Arc<RwLock<ServiceMappingsCollection>>>>,
    pub (crate) links: RwLock<HashMap<TypeId, DependencyLink>>,
    pub (crate) global_scope: Arc<RwLock<GlobalScope>>,
}

impl Debug for DependencyCoreContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependencyCoreContext")
            .field("cycled_component_builders", &self.cycled_component_builders.try_read().unwrap())
            .field("components", &self.components.try_read().unwrap())
            .field("services", 
            &self.services.try_read().unwrap()
                .iter().map(|(id, services)| (id.clone(), services.try_read().unwrap())).collect::<HashMap<_, _>>()
            )
            .field("links", &self.links.try_read().unwrap())
            .field("global_scope", &self.global_scope.try_read().unwrap())
            .finish()
    }
}

impl DependencyCoreContext {
    pub (crate) fn new() -> Self {
        Self {
            components: RwLock::new(HashMap::new()),
            cycled_component_builders: RwLock::new(HashMap::new()),
            services: RwLock::new(HashMap::new()),
            links: RwLock::new(HashMap::new()),
            global_scope: Arc::new(RwLock::new(GlobalScope::new()))
        }
    }
}