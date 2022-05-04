use std::any::Any;

use tokio::sync::RwLock;

use crate::{
    DependencyContext,
    TypeConstructor,
    types::{
        TypeInfo,
        BuildDependencyResult
    }
};

pub (crate) struct ComponentFromInstance<TComponent: Sync + Send + 'static> {
    instance: RwLock<Option<TComponent>>,
}

impl<TComponent: Sync + Send> ComponentFromInstance<TComponent> {
    pub (crate) fn new(instance: TComponent) -> Self { Self { instance: RwLock::new(Some(instance)) } }
}

#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> TypeConstructor for ComponentFromInstance<TComponent> {
    async fn ctor(&self, _: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let instance = self.instance.write().await.take().expect(&format!("Double request registered instance. Expected single request for singleton TypeInfo:[{type_info:?}]", type_info = TypeInfo::from_type::<TComponent>()));
        Ok(Box::new(instance))
    }
}