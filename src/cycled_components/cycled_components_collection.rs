use std::{any::TypeId, collections::HashMap, sync::{Arc, Weak}};

use crate::types::TypeInfo;

use super::{ICycledComponentBuilder, SingletonComponentBuilder, TransientComponentBuilder, ContextDependentComponentBuilder};

pub (crate) struct ComponentCycledComponentPair {
    //pub (crate) component_id: TypeId,
    //pub (crate) cycled_component_id: TypeId, 
    pub (crate) converter: Box<dyn ICycledComponentBuilder>,

    //#[cfg(feature = "debug-type-info")]
    pub (crate) component_type_info: TypeInfo,

    //#[cfg(feature = "debug-type-info")]
    pub (crate) cycled_component_type_info: TypeInfo,
}

impl std::fmt::Debug for ComponentCycledComponentPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("ComponentCycledComponentPair");
            //.field("component_id", &self.component_id)
            //.field("cycled_component_id", &self.cycled_component_id)
        debug_struct.field("converter", &self.converter);
        
        //#[cfg(feature = "debug-type-info")]
        debug_struct.field("component_type_info", &self.component_type_info)
            .field("cycled_component_type_info", &self.cycled_component_type_info);

        debug_struct.finish()
    }
}

impl ComponentCycledComponentPair {
    pub (crate) fn new<TComponent: 'static, TCycledComponent: 'static>(converter: Box<dyn ICycledComponentBuilder>) -> Self {
        Self {
            //component_id: TypeId::of::<TComponent>(),
            //cycled_component_id: TypeId::of::<TCycledComponent>(),
            converter,
            //#[cfg(feature = "debug-type-info")]
            component_type_info: TypeInfo::from_type::<TComponent>(),
            //#[cfg(feature = "debug-type-info")]
            cycled_component_type_info: TypeInfo::from_type::<TCycledComponent>()
        }
    }
}

#[derive(Debug, Default)]
pub (crate) struct ComponentCycledComponentCollection {
    pub (crate) component_cycled_component_pairs_component_idx: HashMap<TypeId, Arc<ComponentCycledComponentPair>>,
    pub (crate) component_cycled_component_pairs_cycled_component_idx: HashMap<TypeId, Arc<ComponentCycledComponentPair>>,
}

impl ComponentCycledComponentCollection {
    #[inline(always)]
    pub (crate) fn add_transient_cycle_builder<TComponent: Sync + Send + 'static>(&mut self) {
        let component_cycled_component_pair = Arc::new(ComponentCycledComponentPair::new::<TComponent, TComponent>(Box::new(TransientComponentBuilder::<TComponent>::new())));

        self.component_cycled_component_pairs_component_idx.insert(TypeId::of::<TComponent>(), component_cycled_component_pair.clone());
        self.component_cycled_component_pairs_cycled_component_idx.insert(TypeId::of::<TComponent>(), component_cycled_component_pair);
    }

    #[inline(always)]
    pub (crate) fn add_singleton_cycle_builder<TComponent: Sync + Send + 'static>(&mut self) {
        let component_cycled_component_pair = Arc::new(ComponentCycledComponentPair::new::<TComponent, Arc<TComponent>>(Box::new(SingletonComponentBuilder::<TComponent>::new())));

        self.component_cycled_component_pairs_component_idx.insert(TypeId::of::<TComponent>(), component_cycled_component_pair.clone());
        self.component_cycled_component_pairs_cycled_component_idx.insert(TypeId::of::<Arc<TComponent>>(), component_cycled_component_pair);
    }

    #[inline(always)]
    pub (crate) fn add_context_dependent_cycle_builder<TComponent: Sync + Send + 'static>(&mut self) {
        let component_cycled_component_pair = Arc::new(ComponentCycledComponentPair::new::<TComponent, Weak<TComponent>>(Box::new(ContextDependentComponentBuilder::<TComponent>::new())));

        self.component_cycled_component_pairs_component_idx.insert(TypeId::of::<TComponent>(), component_cycled_component_pair.clone());
        self.component_cycled_component_pairs_cycled_component_idx.insert(TypeId::of::<Weak<TComponent>>(), component_cycled_component_pair);
    }

    #[inline(always)]
    pub (crate) fn get_by_cycled_component_id(&self, cycled_component_id: &TypeId) -> Option<Arc<ComponentCycledComponentPair>> {
        Some(self.component_cycled_component_pairs_cycled_component_idx.get(&cycled_component_id)?.clone())
    }

    // pub (crate) fn get_by_component_id(&self, component_id: &TypeId) -> Option<Arc<ComponentCycledComponentPair>> {
    //     Some(self.component_cycled_component_pairs_component_idx.get(&component_id)?.clone())
    // }

    #[inline(always)]
    pub (crate) fn delete_by_component<TComponent: 'static>(&mut self) -> Option<Arc<ComponentCycledComponentPair>> {
        let component_cycled_component_pair = self.component_cycled_component_pairs_component_idx.remove(&TypeId::of::<TComponent>())?;
        _ = self.component_cycled_component_pairs_cycled_component_idx.remove(&component_cycled_component_pair.cycled_component_type_info.type_id)?;
        Some(component_cycled_component_pair)
    }
}

