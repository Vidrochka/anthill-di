use std::{
    collections::HashMap,
    any::TypeId,
    sync::Arc,
    fmt::Debug
};

use tokio::sync::RwLock;
use derive_new::new;

use crate::{
    Dependency,
    DependencyLink,
    service::ServicesMappingsCollection,
    ICycledComponentBuilder,
    GlobalScope
};




//#[derive(Debug)]
#[derive(Default, new)]
pub (crate) struct DependencyCoreContext {
    #[new(default)] pub (crate) components: RwLock<HashMap<TypeId, Arc<Dependency>>>,
    #[new(default)] pub (crate) cycled_component_builders: RwLock<HashMap<TypeId, Arc<Box<dyn ICycledComponentBuilder>>>>,
    #[new(default)] pub (crate) services: RwLock<ServicesMappingsCollection>,
    #[new(default)] pub (crate) links: RwLock<HashMap<TypeId, DependencyLink>>,
    #[new(default)] pub (crate) global_scope: Arc<RwLock<GlobalScope>>,
}

impl Debug for DependencyCoreContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependencyCoreContext")
            .field("cycled_component_builders", &self.cycled_component_builders.try_read().unwrap())
            .field("components", &self.components.try_read().unwrap())
            .field("services", &self.services.try_read().unwrap())
            .field("links", &self.links.try_read().unwrap())
            .field("global_scope", &self.global_scope.try_read().unwrap())
            .finish()
    }
}