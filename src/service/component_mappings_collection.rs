use std::{collections::HashMap, any::TypeId, marker::Unsize};
use derive_new::new;

use crate::types::TypeInfo;
use super::{IServiceConstructor, BoxedTraitService, NoLogicService, WeakTraitService, ArcTraitService};

#[derive(Debug, new)]
pub (crate) struct ComponentMappingsCollection {
    #[allow(dead_code)]
    service_type_info: TypeInfo,
    
    #[new(value = "HashMap::new()")]
    service_mappings: HashMap<TypeId, Box<dyn IServiceConstructor>>
}

impl ComponentMappingsCollection {
    pub (crate) fn map_component_as_boxed_trait<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let service = BoxedTraitService::<TComponent, TService>::new();

        if self.service_mappings.contains_key(&service.get_component_info().type_id) {
            return;
        }
    
        self.service_mappings.insert(service.get_component_info().type_id, Box::new(service));
    }

    pub (crate) fn map_component_as_arc_trait<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let service = ArcTraitService::<TComponent, TService>::new();

        if self.service_mappings.contains_key(&service.get_component_info().type_id) {
            return;
        }
    
        self.service_mappings.insert(service.get_component_info().type_id, Box::new(service));
    }

    pub (crate) fn map_component_as_weak_trait<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let service = WeakTraitService::<TComponent, TService>::new();

        if self.service_mappings.contains_key(&service.get_component_info().type_id) {
            return;
        }
    
        self.service_mappings.insert(service.get_component_info().type_id, Box::new(service));
    }

    pub (crate) fn add_mapping_component_to_component<TComponent: Sync + Send + 'static>(&mut self) {
        let service = NoLogicService::<TComponent>::new();

        if self.service_mappings.contains_key(&service.get_component_info().type_id) {
            return;
        }

        self.service_mappings.insert(service.get_component_info().type_id, Box::new(service));
    }

    pub (crate) fn get_nth_service_info(&self, n: usize) -> (&TypeId, &Box<dyn IServiceConstructor>) {
        if self.service_mappings.is_empty() { panic!("empty service collection, incorrect logic"); }

        self.service_mappings.iter().nth(n).expect(&format!("Expected n:[{n}] in range:[0]-[{to}]", to = self.service_mappings.len()))
    }

    pub (crate) fn get_by_type_id(&self, component_type_id: &TypeId) -> &Box<dyn IServiceConstructor> {
        if self.service_mappings.is_empty() { panic!("empty service collection, incorrect logic"); }

        self.service_mappings.get(&component_type_id).expect(&format!("Expected mapping component type_id:[{component_type_id:?}] to service type_id:[{service_type_id:?}]", service_type_id = self.service_type_info.type_id))
    }

    pub (crate) fn get_all_services_info(&self) -> Vec<(&TypeId, &Box<dyn IServiceConstructor>)> {
        if self.service_mappings.is_empty() { panic!("empty service collection, incorrect logic"); }

        self.service_mappings.iter().collect()
    }
}