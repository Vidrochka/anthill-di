use std::{
    fmt::Debug,
    any::{
        TypeId,
        Any
    }
};

use async_trait::async_trait;

use crate::{
    DependencyContext,
    types::BuildDependencyResult
};

pub (crate) struct DependencyType {
    pub (crate) id: TypeId,
    pub (crate) ctor: Box<dyn TypeConstructor>,
}

impl DependencyType {
    pub (crate) fn new(id: TypeId, ctor: Box<dyn TypeConstructor>) -> Self {
        Self { id, ctor, }
    }
}

impl Debug for DependencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DependencyType").field("id", &self.id).field("ctor", &"hidden ctor".to_string()).finish()
    }
}

#[async_trait]
pub trait TypeConstructor where Self: Sync + Send {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
}