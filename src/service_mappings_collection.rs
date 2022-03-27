use std::any::type_name;
use std::{collections::HashMap, any::TypeId, marker::Unsize};

use crate::types::TypeInfo;
use crate::{IServiceConstructor, BoxedTraitService, NoLogicService};

#[derive(Debug)]
pub (crate) struct ServiceMappingsCollection {
    service_type_info: TypeInfo,
    service_mappings: HashMap<TypeId, Box<dyn IServiceConstructor>>
}

impl ServiceMappingsCollection {
    pub (crate) fn new<TService: ?Sized + Sync + Send + 'static>() -> Self {
        Self {
            service_type_info: TypeInfo::new(TypeId::of::<TService>(), type_name::<TService>().to_string()),
            service_mappings: HashMap::new()
        }
    }

    pub (crate) fn add_mapping_component_as_trait_service<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let service = BoxedTraitService::<TComponent, TService>::new();

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

    pub (crate) fn get_service_info(&self) -> &TypeInfo { 
        &self.service_type_info
    }
}