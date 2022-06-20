use std::fmt::Debug;
use std::{
    any::Any,
    future::Future
};

use crate::{
    types::BuildDependencyResult,
    DependencyContext,
    component::ITypeConstructor
};

use derive_new::new;
#[cfg(feature = "blocking")]
use tokio::runtime::Builder;

#[derive(new)]
pub (crate) struct ComponentFromAsyncClosure<TComponent, TFuture, TClosure>
where
    TComponent: Sync + Send + 'static,
    TFuture: Future<Output = BuildDependencyResult<TComponent>>,
    TFuture: Sync + Send,
    TClosure: Fn(DependencyContext) -> TFuture,
    TClosure: Sync + Send + 'static,
{
    closure: TClosure
}

impl<TComponent, TFuture, TClosure> Debug for ComponentFromAsyncClosure<TComponent, TFuture, TClosure>
where
    TComponent: Sync + Send + 'static,
    TFuture: Future<Output = BuildDependencyResult<TComponent>>,
    TFuture: Sync + Send,
    TClosure: Fn(DependencyContext) -> TFuture,
    TClosure: Sync + Send + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentFromAsyncClosure")
            .finish()
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent, TFuture, TClosure> ITypeConstructor for ComponentFromAsyncClosure<TComponent, TFuture, TClosure>
where
    TComponent: Sync + Send + 'static,
    TFuture: Future<Output = BuildDependencyResult<TComponent>>,
    TFuture: Sync + Send,
    TClosure: Fn(DependencyContext) -> TFuture,
    TClosure: Sync + Send + 'static,
{
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx).await?;
        Ok(Box::new(build_result))
    }

    #[cfg(feature = "blocking")]
    fn blocking_ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        let build_result = rt.block_on(async move { (self.closure)(ctx).await })?;
        Ok(Box::new(build_result))
    }
}