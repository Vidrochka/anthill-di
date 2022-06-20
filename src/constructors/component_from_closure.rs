use std::{
    fmt::Debug,
    any::Any,
};

use derive_new::new;

use crate::{
    types::BuildDependencyResult,
    DependencyContext,
    ITypeConstructor
};

trait Closure<TComponent: Sync + Send + 'static> = Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send;

#[derive(new)]
pub (crate) struct ComponentFromClosure<TComponent: Sync + Send + 'static> {
    closure: Box<dyn Closure<TComponent>>
}

impl<TComponent: Sync + Send + 'static> Debug for ComponentFromClosure<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentFromClosure")
            .finish()
    }
}

#[cfg(not(feature = "async-mode"))]
impl<TComponent: Sync + Send + 'static> ITypeConstructor for ComponentFromClosure<TComponent> {
    fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx)?;
        Ok(Box::new(build_result))
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ITypeConstructor for ComponentFromClosure<TComponent> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx)?;
        Ok(Box::new(build_result))
    }

    #[cfg(feature = "blocking")]
    fn blocking_ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let build_result = (self.closure)(ctx)?;
        Ok(Box::new(build_result))
    }
}