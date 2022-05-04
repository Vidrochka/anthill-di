use std::any::Any;

use derive_new::new;

use crate::{
    types::BuildDependencyResult,
    DependencyContext,
    TypeConstructor
};

#[derive(new)]
pub (crate) struct ComponentFromClosure<TComponent: Sync + Send + 'static> {
    closure: Box<dyn Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send>
}

#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> TypeConstructor for ComponentFromClosure<TComponent> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx)?;
        Ok(Box::new(build_result))
    }
}