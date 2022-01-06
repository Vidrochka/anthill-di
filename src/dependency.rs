use std::any::TypeId;

use crate::{
    DependencyLifeCycle,
    DependencyType,
    TypeConstructor
};

#[derive(Debug)]
pub (crate) struct Dependency {
    pub (crate) life_cycle_type: DependencyLifeCycle,
    pub (crate) di_type: DependencyType,
}

impl Dependency {
    pub (crate) fn new_transient<T: 'static>(ctor: Box<dyn TypeConstructor>) -> Self {
        Self {
            life_cycle_type: DependencyLifeCycle::Transient,
            di_type: DependencyType::new(TypeId::of::<T>(), ctor),
        }
    }

    pub (crate) fn new_singleton<T: 'static>(ctor: Box<dyn TypeConstructor>) -> Self {
        Self {
            life_cycle_type: DependencyLifeCycle::Singleton,
            di_type: DependencyType::new(TypeId::of::<T>(), ctor),
        }
    }

    pub (crate) fn new_scoped<T: 'static>(ctor: Box<dyn TypeConstructor>) -> Self {
        Self {
            life_cycle_type: DependencyLifeCycle::Scoped,
            di_type: DependencyType::new(TypeId::of::<T>(), ctor),
        }
    }
}