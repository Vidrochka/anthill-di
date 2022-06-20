use tokio::sync::RwLock;

use crate::{
    DependencyContext,
    types::BuildDependencyResult
};

#[cfg(not(feature = "async-mode"))]
pub trait Constructor where Self: Sized + 'static {
    fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self>;
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
pub trait Constructor where Self: Sized + 'static {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self>;
}

#[cfg(not(feature = "async-mode"))]
impl <T: Constructor + Sized + 'static> Constructor for tokio::sync::RwLock<T> {
    fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(RwLock::new(T::ctor(ctx)?))
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl <T: Constructor + Sized + 'static> Constructor for tokio::sync::RwLock<T> {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(RwLock::new(T::ctor(ctx).await?))
    }
}

#[cfg(not(feature = "async-mode"))]
impl <T: Constructor + Sized + 'static> Constructor for std::sync::RwLock<T> {
    fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(std::sync::RwLock::new(T::ctor(ctx)?))
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl <T: Constructor + Sized + 'static> Constructor for std::sync::RwLock<T> {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self> {
        Ok(std::sync::RwLock::new(T::ctor(ctx).await?))
    }
}