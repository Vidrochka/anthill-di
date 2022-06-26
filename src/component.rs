use std::{
    fmt::Debug,
    any::Any
};

use crate::{
    LifeCycle,
    types::{TypeInfo, BuildDependencyResult}, DependencyContext,
};

pub (crate) struct Component {
    pub (crate) life_cycle_type: LifeCycle,
    //pub (crate) di_type: DependencyType,
    pub (crate) component_type_info: TypeInfo,
    pub (crate) ctor: Box<dyn ITypeConstructor>,
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Component")
            .field("life_cycle_type", &self.life_cycle_type)
            .field("component_type_info", &self.component_type_info)
            .field("ctor",&self.ctor).finish()
    }
}

impl Component {
    pub (crate) fn new<TComponent: 'static>(life_cycle_type: LifeCycle, ctor: Box<dyn ITypeConstructor>) -> Self {
        Self {
            life_cycle_type,
            component_type_info: TypeInfo::from_type::<TComponent>(),
            ctor,
        }
    }
}

#[cfg(not(feature = "async-mode"))]
pub trait ITypeConstructor where Self: Sync + Send + Debug {
    fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
pub trait ITypeConstructor where Self: Sync + Send + Debug {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
    #[cfg(feature = "blocking")]
    fn blocking_ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
}