use std::any::Any;

use crate::{types::{AsyncCallback, BuildDependencyResult}, DependencyContext, TypeConstructor};

use async_trait::async_trait;
use derive_new::new;

#[derive(new)]
pub (crate) struct ComponentFromAsyncClosure<TComponent: Sync + Send + 'static> {
    closure: AsyncCallback<DependencyContext, BuildDependencyResult<TComponent>>
}

#[async_trait]
impl<TComponent: Sync + Send + 'static> TypeConstructor for ComponentFromAsyncClosure<TComponent> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx).await?;
        Ok(Box::new(build_result))
    }
}