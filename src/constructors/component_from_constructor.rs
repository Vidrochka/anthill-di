use std::marker::PhantomData;
use std::any::Any;

use crate::DependencyContext;
use crate::TypeConstructor;
use crate::Constructor;
use crate::types::BuildDependencyResult;

use derive_new::new;

#[derive(new)]
pub (crate) struct ComponentFromConstructor<TComponent: Constructor + Sync + Send> {
    component_phantom_data: PhantomData<TComponent>,
}
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Constructor + Sync + Send> TypeConstructor for ComponentFromConstructor<TComponent> {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let new_component = TComponent::ctor(ctx).await?;
        Ok(Box::new(new_component))
    }
}