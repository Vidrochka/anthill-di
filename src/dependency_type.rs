use std::{
    any::{
        type_name,
        TypeId,
        Any
    },
    fmt::Debug
};

use crate::{
    DependencyContext,
    types::BuildDependencyResult
};

pub (crate) struct DependencyType {
    pub (crate) id: TypeId,
    pub (crate) name: String,
    pub (crate) ctor: Box<dyn TypeConstructor>,
}

impl DependencyType {
    pub (crate) fn new<T: 'static>(ctor: Box<dyn TypeConstructor>) -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: type_name::<T>().to_string(),
            ctor: ctor,
        }
    }
}

impl Debug for DependencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DependencyType").field("id", &self.id).field("ctor", &"hidden ctor".to_string()).finish()
    }
}

#[async_trait_with_sync::async_trait(Sync)]
pub trait TypeConstructor where Self: Sync + Send {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
}