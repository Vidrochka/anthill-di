use std::any::Any;

use crate::{
    DependencyContext,
    ITypeConstructor,
    types::{
        TypeInfo,
        BuildDependencyResult, AnthillRwLock
    }
};

pub (crate) struct ComponentFromInstance<TComponent: Sync + Send + 'static> {
    instance: AnthillRwLock<Option<TComponent>>,
}

impl<TComponent: Sync + Send + 'static> std::fmt::Debug for ComponentFromInstance<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentFromInstance")
            .finish()
    }
}

impl<TComponent: Sync + Send> ComponentFromInstance<TComponent> {
    pub (crate) fn new(instance: TComponent) -> Self { Self { instance: AnthillRwLock::new(Some(instance)) } }
}

#[cfg(not(feature = "async-mode"))]
impl<TComponent: Sync + Send + 'static> ITypeConstructor for ComponentFromInstance<TComponent> {
    fn ctor(&self, _: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let instance = self.instance.write().unwrap().take().expect(&format!("Double request registered instance. Expected single request for singleton TypeInfo:[{type_info:?}]", type_info = TypeInfo::from_type::<TComponent>()));
        Ok(Box::new(instance))
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ITypeConstructor for ComponentFromInstance<TComponent> {
    async fn ctor(&self, _: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let instance = self.instance.write().await.take().expect(&format!("Double request registered instance. Expected single request for singleton TypeInfo:[{type_info:?}]", type_info = TypeInfo::from_type::<TComponent>()));
        Ok(Box::new(instance))
    }

    #[cfg(feature = "blocking")]
    fn blocking_ctor(&self, _: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let instance = self.instance.blocking_write().take().expect(&format!("Double request registered instance. Expected single request for singleton TypeInfo:[{type_info:?}]", type_info = TypeInfo::from_type::<TComponent>()));
        Ok(Box::new(instance))
    }
}