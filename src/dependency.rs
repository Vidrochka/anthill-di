use crate::{
    DependencyLifeCycle,
    DependencyType,
};

use derive_new::new;

#[derive(Debug, new)]
pub (crate) struct Dependency {
    pub (crate) life_cycle_type: DependencyLifeCycle,
    pub (crate) di_type: DependencyType,
}