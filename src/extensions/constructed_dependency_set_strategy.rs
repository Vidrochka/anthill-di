use async_trait::async_trait;

use crate::{
    Constructor,
    DependencyContext,
    constructors::BaseConstructor,
    types::AddDependencyResult
};

#[async_trait]
pub trait ConstructedDependencySetStrategy {
    async fn set_transient<TType: Sync + Send + Constructor>(&self) -> AddDependencyResult<()>;
    async fn set_singleton<TType: Sync + Send + Constructor>(&self) -> AddDependencyResult<()>;
    async fn set_scoped<TType: Sync + Send + Constructor>(&self) -> AddDependencyResult<()>;
}

#[async_trait]
impl ConstructedDependencySetStrategy for DependencyContext {
    async fn set_transient<TType: Sync + Send + Constructor>(&self) -> AddDependencyResult<()> {
        self.add_transient::<TType>(Box::new(BaseConstructor::new::<TType>())).await
    }

    async fn set_singleton<TType: Sync + Send + Constructor>(&self) -> AddDependencyResult<()> {
        self.add_singleton::<TType>(Box::new(BaseConstructor::new::<TType>())).await
    }

    async fn set_scoped<TType: Sync + Send + Constructor>(&self) -> AddDependencyResult<()> {
        self.add_scoped::<TType>(Box::new(BaseConstructor::new::<TType>())).await
    }
}