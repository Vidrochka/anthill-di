use async_trait::async_trait;

use crate::{
    Constructor,
    DependencyContext,
    constructors::BaseConstructor,
    types::AddDependencyResult
};

#[async_trait(?Send)]
pub trait ConstructedDependencySetStrategy {
    async fn set_transient<TType: Constructor>(&self) -> AddDependencyResult<()>;
    async fn set_singleton<TType: Constructor>(&self) -> AddDependencyResult<()>;
    async fn set_scoped<TType: Constructor>(&self) -> AddDependencyResult<()>;
}

#[async_trait(?Send)]
impl ConstructedDependencySetStrategy for DependencyContext {
    async fn set_transient<TType: Constructor>(&self) -> AddDependencyResult<()> {
        self.add_transient::<TType>(Box::new(BaseConstructor::new::<TType>())).await
    }

    async fn set_singleton<TType: Constructor>(&self) -> AddDependencyResult<()> {
        self.add_singleton::<TType>(Box::new(BaseConstructor::new::<TType>())).await
    }

    async fn set_scoped<TType: Constructor>(&self) -> AddDependencyResult<()> {
        self.add_scoped::<TType>(Box::new(BaseConstructor::new::<TType>())).await
    }
}