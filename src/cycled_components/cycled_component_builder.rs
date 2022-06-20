use core::fmt::Debug;
use core::any::TypeId;
use std::{
    any::Any,
    sync::Arc,
    marker::PhantomData
};

use crate::{
    types::{
        BuildDependencyResult,
        TypeInfo, AnthillRwLock
    },
    core_context::CoreContext,
    LocalContext,
    DependencyContextId,
    DependencyContext
};

#[cfg(not(feature = "async-mode"))]
pub (crate) trait ICycledComponentBuilder where Self: Debug + Sync + Send + 'static {    
    fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
pub (crate) trait ICycledComponentBuilder where Self: Debug + Sync + Send + 'static {    
    async fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
    #[cfg(feature = "blocking")]
    fn blocking_build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
}

// ----------- Maby move to self file -----------
pub (crate) struct SingletonComponentBuilder<TComponent: Sync + Send + 'static> {
    component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: Sync + Send + 'static> Debug for SingletonComponentBuilder<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingletonComponentBuilder").field("component_phantom_data", &self.component_phantom_data).finish()
    }
}

impl<TComponent: Sync + Send + 'static> SingletonComponentBuilder<TComponent> {
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
        }
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for SingletonComponentBuilder<TComponent> {
    async fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let singleton_component_type_id = TypeId::of::<Arc<TComponent>>();
        let global_context_read_guard = core_context.global_context.read().await;
    
        if let Some(singleton_component_instance) = global_context_read_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        drop(global_context_read_guard); // дедлок!!!!!!!!!!!!!!!!!!!
        let mut global_context_write_guard = core_context.global_context.write().await;
    
        if let Some(singleton_component_instance) = global_context_write_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        let new_singleton = Arc::new(AnthillRwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        global_context_write_guard.singletons.insert(singleton_component_type_id, new_singleton.clone());
    
        let mut new_singleton_write_guard = new_singleton.write().await;
        drop(global_context_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ
    
        let component_type_id = TypeId::of::<TComponent>();
    
        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());
    
        let component = core_context.components.read().await.get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();
    
        let new_component_instance = component.ctor.ctor(dependency_context).await?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));
    
        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_singleton_write_guard.insert(new_component_instance_ref.clone());

        return Ok(Box::new(new_component_instance_ref) as Box<dyn Any + Sync + Send>);
    }

    #[cfg(feature = "blocking")]
    fn blocking_build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let singleton_component_type_id = TypeId::of::<Arc<TComponent>>();
        let global_context_read_guard = core_context.global_context.blocking_read();
    
        if let Some(singleton_component_instance) = global_context_read_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.blocking_read().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        drop(global_context_read_guard); // дедлок!!!!!!!!!!!!!!!!!!!
        let mut global_context_write_guard = core_context.global_context.blocking_write();
    
        if let Some(singleton_component_instance) = global_context_write_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.blocking_read().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        let new_singleton = Arc::new(AnthillRwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        global_context_write_guard.singletons.insert(singleton_component_type_id, new_singleton.clone());
    
        let mut new_singleton_write_guard = new_singleton.blocking_write();
        drop(global_context_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ
    
        let component_type_id = TypeId::of::<TComponent>();
    
        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());
    
        let component = core_context.components.blocking_read().get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();
    
        let component_ref = component.clone();
        let new_component_instance = component_ref.ctor.blocking_ctor(dependency_context)?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));
    
        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_singleton_write_guard.insert(new_component_instance_ref.clone());

        return Ok(Box::new(new_component_instance_ref) as Box<dyn Any + Sync + Send>);
    }
}

#[cfg(not(feature = "async-mode"))]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for SingletonComponentBuilder<TComponent> {
    fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let singleton_component_type_id = TypeId::of::<Arc<TComponent>>();
        let global_context_read_guard = core_context.global_context.read().unwrap();
    
        if let Some(singleton_component_instance) = global_context_read_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.read().unwrap().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        drop(global_context_read_guard); // дедлок!!!!!!!!!!!!!!!!!!!
        let mut global_context_write_guard = core_context.global_context.write().unwrap();
    
        if let Some(singleton_component_instance) = global_context_write_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.read().unwrap().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        let new_singleton = Arc::new(AnthillRwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        global_context_write_guard.singletons.insert(singleton_component_type_id, new_singleton.clone());
    
        let mut new_singleton_write_guard = new_singleton.write().unwrap();
        drop(global_context_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ
    
        let component_type_id = TypeId::of::<TComponent>();
    
        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());
    
        let component = core_context.components.read().unwrap().get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();
    
        let new_component_instance = component.ctor.ctor(dependency_context)?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));
    
        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_singleton_write_guard.insert(new_component_instance_ref.clone());

        return Ok(Box::new(new_component_instance_ref) as Box<dyn Any + Sync + Send>);
    }
}

// ----------- Maby move to self file -----------
pub (crate) struct ContextDependentComponentBuilder<TComponent: Sync + Send + 'static> {
    component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: Sync + Send + 'static> Debug for ContextDependentComponentBuilder<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextDependentComponentBuilder").field("component_phantom_data", &self.component_phantom_data).finish()
    }
}

impl<TComponent: Sync + Send + 'static> ContextDependentComponentBuilder<TComponent> {
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
        }
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for ContextDependentComponentBuilder<TComponent> {
    async fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let context_dependent_component_type_id = TypeId::of::<Arc<TComponent>>();
        let local_context_read_guard = local_context.local_context.read().await;

        if let Some(context_dependent_component_instance_ref) = local_context_read_guard.get(&context_dependent_component_type_id) {
            let context_dependent_component_instance: Arc<TComponent> = context_dependent_component_instance_ref.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect context dependent type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(Arc::downgrade(&context_dependent_component_instance)) as Box<dyn Any + Sync + Send>);
        }

        drop(local_context_read_guard);
        let mut local_context_write_guard = local_context.local_context.write().await;

        if let Some(context_dependent_component_instance_ref) = local_context_write_guard.get(&context_dependent_component_type_id) {
            let context_dependent_component_instance: Arc<TComponent> = context_dependent_component_instance_ref.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect context dependent type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));

            return Ok(Box::new(context_dependent_component_instance) as Box<dyn Any + Sync + Send>);
        }

        let new_context_dependent_any_ref = Arc::new(AnthillRwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        local_context_write_guard.insert(context_dependent_component_type_id, new_context_dependent_any_ref.clone());

        let mut new_context_dependent_any_ref_write_guard = new_context_dependent_any_ref.write().await;
        drop(local_context_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ

        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());

        let component = core_context.components.read().await.get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();

        let new_component_instance = component.ctor.ctor(dependency_context).await?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));

        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_context_dependent_any_ref_write_guard.insert(new_component_instance_ref.clone());

        Ok(Box::new(Arc::downgrade(&new_component_instance_ref)) as Box<dyn Any + Sync + Send>)
    }

    #[cfg(feature = "blocking")]
    fn blocking_build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let context_dependent_component_type_id = TypeId::of::<Arc<TComponent>>();
        let local_context_read_guard = local_context.local_context.blocking_read();

        if let Some(context_dependent_component_instance_ref) = local_context_read_guard.get(&context_dependent_component_type_id) {
            let context_dependent_component_instance: Arc<TComponent> = context_dependent_component_instance_ref.blocking_read().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect context dependent type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(Arc::downgrade(&context_dependent_component_instance)) as Box<dyn Any + Sync + Send>);
        }

        drop(local_context_read_guard);
        let mut local_context_write_guard = local_context.local_context.blocking_write();

        if let Some(context_dependent_component_instance_ref) = local_context_write_guard.get(&context_dependent_component_type_id) {
            let context_dependent_component_instance: Arc<TComponent> = context_dependent_component_instance_ref.blocking_read().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect context dependent type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));

            return Ok(Box::new(context_dependent_component_instance) as Box<dyn Any + Sync + Send>);
        }

        let new_context_dependent_any_ref = Arc::new(AnthillRwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        local_context_write_guard.insert(context_dependent_component_type_id, new_context_dependent_any_ref.clone());

        let mut new_context_dependent_any_ref_write_guard = new_context_dependent_any_ref.blocking_write();
        drop(local_context_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ

        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());

        let component = core_context.components.blocking_read().get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();

        let component_ref = component.clone();
        let new_component_instance = component_ref.ctor.blocking_ctor(dependency_context)?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));

        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_context_dependent_any_ref_write_guard.insert(new_component_instance_ref.clone());

        Ok(Box::new(Arc::downgrade(&new_component_instance_ref)) as Box<dyn Any + Sync + Send>)
    }
}

#[cfg(not(feature = "async-mode"))]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for ContextDependentComponentBuilder<TComponent> {
    fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let context_dependent_component_type_id = TypeId::of::<Arc<TComponent>>();
        let local_context_read_guard = local_context.local_context.read().unwrap();

        if let Some(context_dependent_component_instance_ref) = local_context_read_guard.get(&context_dependent_component_type_id) {
            let context_dependent_component_instance: Arc<TComponent> = context_dependent_component_instance_ref.read().unwrap().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect context dependent type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));
            return Ok(Box::new(Arc::downgrade(&context_dependent_component_instance)) as Box<dyn Any + Sync + Send>);
        }

        drop(local_context_read_guard);
        let mut local_context_write_guard = local_context.local_context.write().unwrap();

        if let Some(context_dependent_component_instance_ref) = local_context_write_guard.get(&context_dependent_component_type_id) {
            let context_dependent_component_instance: Arc<TComponent> = context_dependent_component_instance_ref.read().unwrap().as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect context dependent type expected type_info:[{:?}]", TypeInfo::from_type::<Arc<TComponent>>()));

            return Ok(Box::new(context_dependent_component_instance) as Box<dyn Any + Sync + Send>);
        }

        let new_context_dependent_any_ref = Arc::new(AnthillRwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        local_context_write_guard.insert(context_dependent_component_type_id, new_context_dependent_any_ref.clone());

        let mut new_context_dependent_any_ref_write_guard = new_context_dependent_any_ref.write().unwrap();
        drop(local_context_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ

        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());

        let component = core_context.components.read().unwrap().get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();

        let new_component_instance = component.ctor.ctor(dependency_context)?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));

        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_context_dependent_any_ref_write_guard.insert(new_component_instance_ref.clone());

        Ok(Box::new(Arc::downgrade(&new_component_instance_ref)) as Box<dyn Any + Sync + Send>)
    }
}

// ----------- Maby move to self file -----------
pub (crate) struct TransientComponentBuilder<TComponent: Sync + Send + 'static> {
    component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: Sync + Send + 'static> Debug for TransientComponentBuilder<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransientComponentBuilder").field("component_phantom_data", &self.component_phantom_data).finish()
    }
}

impl<TComponent: Sync + Send + 'static> TransientComponentBuilder<TComponent> {
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
        }
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for TransientComponentBuilder<TComponent> {
    async fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());

        let component = core_context.components.read().await.get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();

        let new_component_instance = component.ctor.ctor(dependency_context).await?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));

        Ok(new_component_instance as Box<dyn Any + Sync + Send>)
    }

    #[cfg(feature = "blocking")]
    fn blocking_build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());

        let component = core_context.components.blocking_read().get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();

        let component_ref = component.clone();
        let new_component_instance = component_ref.ctor.blocking_ctor(dependency_context)?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));

        Ok(new_component_instance as Box<dyn Any + Sync + Send>)
    }
}

#[cfg(not(feature = "async-mode"))]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for TransientComponentBuilder<TComponent> {
    fn build(&self, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, core_context.clone(), local_context.clone());

        let component = core_context.components.read().unwrap().get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency type_info:[{:?}]", TypeInfo::from_type::<TComponent>()))
            .clone();

        let new_component_instance = component.ctor.ctor(dependency_context)?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_info:[{:?}] find type_info:[{:?}]",
                TypeInfo::from_type::<TComponent>(),
                component.component_type_info,
            ));

        Ok(new_component_instance as Box<dyn Any + Sync + Send>)
    }
}