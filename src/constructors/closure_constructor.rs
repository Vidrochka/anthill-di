use std::any::Any;

use async_trait::async_trait;

use crate::{
    DependencyContext,
    types::{
        BuildDependencyResult,
        AsyncCallback
    },
    TypeConstructor,
};

pub struct ClosureConstructor<TType: Sync + Send + 'static>  {
    async_ctor: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>,
}

impl<TType: Sync + Send + 'static> ClosureConstructor<TType> {
    pub fn new(closure: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> Self {
        Self { async_ctor: closure }
    }
}

#[async_trait]
impl<T: Sync + Send + 'static> TypeConstructor for ClosureConstructor<T> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        Ok(Box::new((self.async_ctor)(ctx).await?) as Box<dyn Any + Sync + Send>) 
    }
}