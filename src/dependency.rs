use crate::{
    DependencyLifeCycle,
    DependencyType,
};

#[derive(Debug)]
pub (crate) struct Dependency {
    pub (crate) life_cycle_type: DependencyLifeCycle,
    pub (crate) di_type: DependencyType,
}

impl Dependency {
    pub (crate) fn new(life_cycle_type: DependencyLifeCycle, di_type: DependencyType) -> Self {
        Self {
            life_cycle_type,
            di_type,
        }
    }
}