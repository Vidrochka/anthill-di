use std::{
    any::TypeId,
    collections::{
        HashMap,
        HashSet
    },
    marker::Unsize,
    sync::{
        Arc,
        Weak
    }
};

#[cfg(feature = "debug-type-info")]
use crate::types::TypeInfo;

use super::{IServiceConstructor, SelfMappingService, BoxedTraitService, ArcTraitService, WeakTraitService};

#[derive(Debug)]
pub (crate) struct CycledComponentServicePair {
    pub (crate) cycled_component_id: TypeId,
    pub (crate) service_id: TypeId,
    pub (crate) converter: Box<dyn IServiceConstructor>,

    #[allow(dead_code)]
    #[cfg(feature = "debug-type-info")]
    debug_cycled_component_type_info: TypeInfo,

    #[allow(dead_code)]
    #[cfg(feature = "debug-type-info")]
    debug_service_type_info: TypeInfo,
}

impl Eq for CycledComponentServicePair {}

impl PartialEq for CycledComponentServicePair {
    fn eq(&self, other: &Self) -> bool {
        self.cycled_component_id == other.cycled_component_id && self.service_id == other.service_id
    }
}

impl std::hash::Hash for CycledComponentServicePair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cycled_component_id.hash(state);
        self.service_id.hash(state);
    }
}

impl CycledComponentServicePair {
    pub (crate) fn new<TComponent: 'static, TService: ?Sized + 'static>(converter: Box<dyn IServiceConstructor>) -> Self {
        Self {
            service_id: TypeId::of::<TService>(),
            cycled_component_id: TypeId::of::<TComponent>(),
            converter,
            #[cfg(feature = "debug-type-info")]
            debug_cycled_component_type_info: TypeInfo::from_type::<TComponent>(),
            #[cfg(feature = "debug-type-info")]
            debug_service_type_info: TypeInfo::from_type::<TService>(),
        }
    }
}

#[derive(Debug, Default)]
pub (crate) struct CycledComponentServiceCollection {
    pub (crate) services_search_idx: HashMap<TypeId, HashSet<Arc<CycledComponentServicePair>>>,
    pub (crate) cycled_components_search_idx: HashMap<TypeId, HashSet<Arc<CycledComponentServicePair>>>,
}

impl CycledComponentServiceCollection {
    #[inline(always)]
    pub (crate) fn add_mapping_as_self<TComponent: Sync + Send + 'static>(&mut self) {
        let component_service_pair = Arc::new(CycledComponentServicePair::new::<TComponent, TComponent>(Box::new(SelfMappingService::<TComponent>::new())));

        let service_search_idx = self.services_search_idx.entry(TypeId::of::<TComponent>()).or_insert(HashSet::new());
        service_search_idx.insert(component_service_pair.clone());

        let component_search_idx = self.cycled_components_search_idx.entry(TypeId::of::<TComponent>()).or_insert(HashSet::new());
        component_search_idx.insert(component_service_pair);
    }

    #[inline(always)]
    pub (crate) fn add_mapping_as_transient<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let component_service_pair = Arc::new(CycledComponentServicePair::new::<TComponent, Box<TService>>(Box::new(BoxedTraitService::<TComponent, TService>::new())));
        
        let service_search_idx = self.services_search_idx.entry(TypeId::of::<Box<TService>>()).or_insert(HashSet::new());
        service_search_idx.insert(component_service_pair.clone());

        let component_search_idx = self.cycled_components_search_idx.entry(TypeId::of::<TComponent>()).or_insert(HashSet::new());
        component_search_idx.insert(component_service_pair);
    }

    #[inline(always)]
    pub (crate) fn add_mapping_as_singleton<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let component_service_pair = Arc::new(CycledComponentServicePair::new::<Arc<TComponent>, Arc<TService>>(Box::new(ArcTraitService::<TComponent, TService>::new())));
        
        let service_search_idx = self.services_search_idx.entry(TypeId::of::<Arc<TService>>()).or_insert(HashSet::new());
        service_search_idx.insert(component_service_pair.clone());

        let component_search_idx = self.cycled_components_search_idx.entry(TypeId::of::<Arc<TComponent>>()).or_insert(HashSet::new());
        component_search_idx.insert(component_service_pair);
    }

    #[inline(always)]
    pub (crate) fn add_mapping_as_context_dependent<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&mut self) where TComponent: Unsize<TService> {
        let component_service_pair = Arc::new(CycledComponentServicePair::new::<Weak<TComponent>, Weak<TService>>(Box::new(WeakTraitService::<TComponent, TService>::new())));
        
        let service_search_idx = self.services_search_idx.entry(TypeId::of::<Weak<TService>>()).or_insert(HashSet::new());
        service_search_idx.insert(component_service_pair.clone());

        let component_search_idx = self.cycled_components_search_idx.entry(TypeId::of::<Weak<TComponent>>()).or_insert(HashSet::new());
        component_search_idx.insert(component_service_pair);
    }

    #[inline(always)]
    pub (crate) fn get_nth_by_service_type<TService: 'static>(&self, n: usize) -> Option<Arc<CycledComponentServicePair>> {
        let idxes = self.services_search_idx.get(&TypeId::of::<TService>())?;
        let idx = idxes.iter().nth(n)?;

        Some(idx.clone())
    }

    #[inline(always)]
    pub (crate) fn get_all_by_service_type<TService: 'static>(&self) -> Option<Vec<Arc<CycledComponentServicePair>>> {
        let idxes = self.services_search_idx.get(&TypeId::of::<TService>())?;
        Some(idxes.iter().map(|idx| idx.clone() ).collect())
    }

    #[inline(always)]
    pub (crate) fn get_all_by_service_type_with_cycled_component_id<TService: 'static>(&self, component_id: TypeId) -> Option<Arc<CycledComponentServicePair>> {
        let service_idxes = self.services_search_idx.get(&TypeId::of::<TService>())?;
        let component_idxes = self.cycled_components_search_idx.get(&component_id)?;
        let idx = service_idxes.intersection(&component_idxes).nth(0)?;

        Some(idx.clone())
    }

    #[inline(always)]
    pub (crate) fn delete_cycled_component(&mut self, cycled_component_type_id: &TypeId) -> Option<Vec<Arc<CycledComponentServicePair>>> {
        let cycled_component_idxes = self.cycled_components_search_idx.remove(cycled_component_type_id)?;

        cycled_component_idxes.iter().for_each(|component_idx| {
            self.services_search_idx.get_mut(&component_idx.service_id)
                .expect("Service idx not found, but component service pair exist")
                .retain(|service_idx| service_idx != component_idx)
        });

        Some(cycled_component_idxes.into_iter().collect())
    }

    #[inline(always)]
    pub (crate) fn is_service_exist(&self, type_id: &TypeId) -> bool {
        self.services_search_idx.contains_key(type_id)
    }
}