use std::{any::Any, future::Future};

use crate::{types::BuildDependencyResult, DependencyContext, TypeConstructor};

use async_trait::async_trait;
use derive_new::new;

#[derive(new)]
pub (crate) struct ComponentFromAsyncClosure<TComponent, TFuture, TClosure>
where
    TComponent: Sync + Send + 'static,
    TFuture: Future<Output = BuildDependencyResult<TComponent>>,
    TFuture: Sync + Send,
    TClosure: Fn(DependencyContext) -> TFuture,
    TClosure: Sync + Send,
{
    closure: TClosure
}

#[async_trait]
impl<TComponent, TFuture, TClosure> TypeConstructor for ComponentFromAsyncClosure<TComponent, TFuture, TClosure>
where
    TComponent: Sync + Send + 'static,
    TFuture: Future<Output = BuildDependencyResult<TComponent>>,
    TFuture: Sync + Send,
    TClosure: Fn(DependencyContext) -> TFuture,
    TClosure: Sync + Send,
{
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx).await?;
        Ok(Box::new(build_result))
    }
}