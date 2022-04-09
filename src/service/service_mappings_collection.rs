use std::marker::Unsize;
use std::sync::Weak;
use std::{collections::HashMap, any::TypeId, sync::Arc};

use tokio::sync::RwLock;
use derive_new::new;

use crate::types::TypeInfo;

use super::ComponentMappingsCollection;

#[derive(Default, new)]
pub (crate) struct ServicesMappingsCollection {
    #[new(default)]
    services: HashMap<TypeId, Arc<RwLock<ComponentMappingsCollection>>>
}

impl std::fmt::Debug for ServicesMappingsCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServicesMappingsCollection").field("services", &self.services.iter()
            .map(|(id, services)| (id.clone(), services.try_read().unwrap())).collect::<HashMap<_, _>>())
            .finish()
    }
}

impl ServicesMappingsCollection {
    pub (crate) async fn add_no_mappings<TComponent: Sync + Send + 'static>(&mut self) {
        let type_info = TypeInfo::from_type::<TComponent>();

        if !self.services.contains_key(&type_info.type_id) {
            let new_component_mappings_collection = ComponentMappingsCollection::new(type_info.clone());
            self.services.insert(type_info.type_id.clone(), Arc::new(RwLock::new(new_component_mappings_collection)));
        }

        let component_mappings = self.services.get_mut(&type_info.type_id).unwrap().clone();
        component_mappings.write().await.add_mapping_component_to_component::<TComponent>();
    }

    pub (crate) async fn add_transient<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let type_info = TypeInfo::from_type::<Box<TService>>();

        if !self.services.contains_key(&type_info.type_id) {
            let new_component_mappings_collection = ComponentMappingsCollection::new(type_info.clone());
            self.services.insert(type_info.type_id.clone(), Arc::new(RwLock::new(new_component_mappings_collection)));
        }

        let component_mappings = self.services.get_mut(&type_info.type_id).unwrap().clone();
        component_mappings.write().await.map_component_as_boxed_trait::<TComponent, TService>();
    }

    pub (crate) async fn add_singleton<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let type_info = TypeInfo::from_type::<Arc<TService>>();

        if !self.services.contains_key(&type_info.type_id) {
            let new_component_mappings_collection = ComponentMappingsCollection::new(type_info.clone());
            self.services.insert(type_info.type_id.clone(), Arc::new(RwLock::new(new_component_mappings_collection)));
        }

        let component_mappings = self.services.get_mut(&type_info.type_id).unwrap().clone();
        component_mappings.write().await.map_component_as_arc_trait::<TComponent, TService>();
    }

    pub (crate) async fn add_scoped<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let type_info = TypeInfo::from_type::<Weak<TService>>();

        if !self.services.contains_key(&type_info.type_id) {
            let new_component_mappings_collection = ComponentMappingsCollection::new(type_info.clone());
            self.services.insert(type_info.type_id.clone(), Arc::new(RwLock::new(new_component_mappings_collection)));
        }

        let component_mappings = self.services.get_mut(&type_info.type_id).unwrap().clone();
        component_mappings.write().await.map_component_as_weak_trait::<TComponent, TService>();
    }

    pub (crate) fn get<TService: 'static>(&self) -> Option<Arc<RwLock<ComponentMappingsCollection>>> {
        let service_id = TypeId::of::<TService>();
        if let Some(services) = self.services.get(&service_id) {
            return Some(services.clone())
        } else {
            return None
        }
    }
}