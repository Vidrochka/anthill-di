use std::marker::PhantomData;
use std::any::Any;

use async_trait::async_trait;

use crate::DependencyContext;
use crate::TypeConstructor;
use crate::Constructor;
use crate::types::BuildDependencyResult;


pub (crate) struct ComponentWithConstructor<TComponent: Constructor + Sync + Send> {
    component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: Constructor + Sync + Send> ComponentWithConstructor<TComponent> {
    #[must_use] pub (crate) fn new() -> Self { Self { component_phantom_data: PhantomData } }
}

#[async_trait]
impl<TComponent: Constructor + Sync + Send> TypeConstructor for ComponentWithConstructor<TComponent> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let new_component = TComponent::ctor(ctx).await?;
        Ok(Box::new(new_component))
    }
}