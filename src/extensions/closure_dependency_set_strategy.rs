use async_trait::async_trait;

use crate::{
    DependencyContext,
    constructors::ClosureConstructor,
    types::{
        AsyncCallback,
        BuildDependencyResult,
        AddDependencyResult
    }
};

#[async_trait(?Send)]
pub trait ClosureDependencySetStrategy {
    async fn set_transient_closure<TType: 'static>(&self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> AddDependencyResult<()>;
    async fn set_singleton_closure<TType: 'static>(&self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> AddDependencyResult<()>;
    async fn set_scoped_closure<TType: 'static>(&self, closure:  AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> AddDependencyResult<()>;
}

#[async_trait(?Send)]
impl ClosureDependencySetStrategy for DependencyContext {
    async fn set_transient_closure<TType: 'static>(&self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> AddDependencyResult<()> {
        self.add_transient::<TType>(Box::new(ClosureConstructor::<TType>::new(closure))).await
    }

    async fn set_singleton_closure<TType: 'static>(&self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> AddDependencyResult<()> {
        self.add_singleton::<TType>(Box::new(ClosureConstructor::<TType>::new(closure))).await
    }

    async fn set_scoped_closure<TType: 'static>(&self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TType>>) -> AddDependencyResult<()> {
        self.add_scoped::<TType>(Box::new(ClosureConstructor::<TType>::new(closure))).await
    }
}