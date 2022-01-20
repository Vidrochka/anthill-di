use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    DependencyContext,
    types::BuildDependencyResult
};

#[async_trait]
pub trait Constructor where Self: Sized + 'static {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self>;
}

#[async_trait]
impl <T: Constructor + Sized + 'static> Constructor for tokio::sync::RwLock<T> {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(RwLock::new(T::ctor(ctx).await?))
    }
}

#[async_trait]
impl <T: Constructor + Sized + 'static> Constructor for std::sync::RwLock<T> {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(std::sync::RwLock::new(T::ctor(ctx).await?))
    }
}