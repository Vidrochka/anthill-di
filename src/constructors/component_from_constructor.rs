use std::{
    fmt::Debug,
    marker::PhantomData,
    any::Any
};

use crate::{
    DependencyContext,
    ITypeConstructor,
    Constructor,
    types::BuildDependencyResult
};

use derive_new::new;
#[cfg(feature = "blocking")]
use tokio::runtime::Builder;

#[derive(new)]
pub (crate) struct ComponentFromConstructor<TComponent: Constructor + Sync + Send> {
    component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: Constructor + Sync + Send> Debug for ComponentFromConstructor<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentFromConstructor")
            .finish()
    }
}

#[cfg(not(feature = "async-mode"))]
impl<TComponent: Constructor + Sync + Send> ITypeConstructor for ComponentFromConstructor<TComponent> {
    fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let new_component = TComponent::ctor(ctx)?;
        Ok(Box::new(new_component))
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Constructor + Sync + Send> ITypeConstructor for ComponentFromConstructor<TComponent> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let new_component = TComponent::ctor(ctx).await?;
        Ok(Box::new(new_component))
    }

    #[cfg(feature = "blocking")]
    fn blocking_ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        let new_component = rt.block_on(async move { TComponent::ctor(ctx).await })?;
        Ok(Box::new(new_component))
    }
}