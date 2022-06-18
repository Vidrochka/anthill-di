use std::{
    any::TypeId,
    collections::{HashMap, HashSet}, marker::Unsize, sync::{Arc, Weak}
};

use crate::types::TypeInfo;

use super::{IServiceConstructor, SelfMappingService, BoxedTraitService, ArcTraitService, WeakTraitService};

#[derive(Debug)]
pub (crate) struct ComponentServicePair {
    pub (crate) component_id: TypeId,
    pub (crate) service_id: TypeId,
    pub (crate) converter: Box<dyn IServiceConstructor>,

    #[cfg(feature = "debug-type-info")]
    debug_component_type_info: TypeInfo,

    #[cfg(feature = "debug-type-info")]
    debug_service_type_info: TypeInfo,
}

impl ComponentServicePair {
    pub (crate) fn new<TComponent: 'static, TService: ?Sized + 'static>(converter: Box<dyn IServiceConstructor>) -> Self {
        Self {
            service_id: TypeId::of::<TService>(),
            component_id: TypeId::of::<TComponent>(),
            converter,
            #[cfg(feature = "debug-type-info")]
            debug_component_type_info: TypeInfo::from_type::<TComponent>(),
            #[cfg(feature = "debug-type-info")]
            debug_service_type_info: TypeInfo::from_type::<TService>(),
        }
    }
}

#[derive(Debug, Default)]
pub (crate) struct ComponentServiceCollection {
    pub (crate) component_service_pairs: Vec<Arc<ComponentServicePair>>,
    pub (crate) services_search_idx: HashMap<TypeId, HashSet<usize>>,
    pub (crate) components_search_idx: HashMap<TypeId, HashSet<usize>>,
}

impl ComponentServiceCollection {
    pub (crate) async fn add_mapping_as_self<TComponent: Sync + Send + 'static>(&mut self) {
        self.component_service_pairs.push(Arc::new(ComponentServicePair::new::<TComponent, TComponent>(Box::new(SelfMappingService::<TComponent>::new()))));
        let idx = self.component_service_pairs.len() - 1;

        let service_search_idx = self.services_search_idx.entry(TypeId::of::<TComponent>()).or_insert(HashSet::new());
        service_search_idx.insert(idx);

        let component_search_idx = self.components_search_idx.entry(TypeId::of::<TComponent>()).or_insert(HashSet::new());
        component_search_idx.insert(idx);
    }

    pub (crate) async fn add_mapping_as_transient<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        self.component_service_pairs.push(Arc::new(ComponentServicePair::new::<TComponent, Box<TService>>(Box::new(BoxedTraitService::<TComponent, TService>::new()))));
        let idx = self.component_service_pairs.len() - 1;
        
        let service_search_idx = self.services_search_idx.entry(TypeId::of::<Box<TService>>()).or_insert(HashSet::new());
        service_search_idx.insert(idx);

        let component_search_idx = self.components_search_idx.entry(TypeId::of::<TComponent>()).or_insert(HashSet::new());
        component_search_idx.insert(idx);
    }

    pub (crate) async fn add_mapping_as_singleton<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        self.component_service_pairs.push(Arc::new(ComponentServicePair::new::<Arc<TComponent>, Arc<TService>>(Box::new(ArcTraitService::<TComponent, TService>::new()))));
        let idx = self.component_service_pairs.len() - 1; 
        
        let service_search_idx = self.services_search_idx.entry(TypeId::of::<Arc<TService>>()).or_insert(HashSet::new());
        service_search_idx.insert(idx);

        let component_search_idx = self.components_search_idx.entry(TypeId::of::<Arc<TComponent>>()).or_insert(HashSet::new());
        component_search_idx.insert(idx);
    }

    pub (crate) async fn add_mapping_as_scoped<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        self.component_service_pairs.push(Arc::new(ComponentServicePair::new::<Weak<TComponent>, Weak<TService>>(Box::new(WeakTraitService::<TComponent, TService>::new()))));
        let idx = self.component_service_pairs.len() - 1; 
        
        let service_search_idx = self.services_search_idx.entry(TypeId::of::<Weak<TService>>()).or_insert(HashSet::new());
        service_search_idx.insert(idx);

        let component_search_idx = self.components_search_idx.entry(TypeId::of::<Weak<TComponent>>()).or_insert(HashSet::new());
        component_search_idx.insert(idx);
    }

    pub (crate) fn get_nth_by_service_type<TService: 'static>(&self, n: usize) -> Option<Arc<ComponentServicePair>> {
        let idxes = self.services_search_idx.get(&TypeId::of::<TService>())?;
        let idx = idxes.iter().nth(0)?;
        Some(self.component_service_pairs.iter().nth(*idx).expect("Component service pair not founc, but idx exist").clone())
    }

    pub (crate) fn get_all_by_service_type<TService: 'static>(&self) -> Option<Vec<Arc<ComponentServicePair>>> {
        let idxes = self.services_search_idx.get(&TypeId::of::<TService>())?;
        Some(idxes.iter().map(|idx| self.component_service_pairs.iter().nth(*idx).expect("Component service pair not founc, but idx exist").clone() ).collect())
    }

    pub (crate) fn get_all_by_service_type_with_component_id<TService: 'static>(&self, component_id: TypeId) -> Option<Arc<ComponentServicePair>> {
        let service_idxes = self.services_search_idx.get(&TypeId::of::<TService>())?;
        let component_idxes = self.components_search_idx.get(&component_id)?;
        let idx = service_idxes.intersection(&component_idxes).nth(0)?;

        Some(self.component_service_pairs.iter().nth(*idx).expect("Component service pair not founc, but idx exist").clone())
    }
}