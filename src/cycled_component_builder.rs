use tokio::sync::RwLock;
use core::fmt::Debug;
use core::any::TypeId;
use std::{any::{Any, type_name}, sync::{Arc, Weak}, marker::PhantomData};
use derive_new::new;

use crate::{
    types::{
        BuildDependencyResult,
        TypeInfo
    },
    core_context::DependencyCoreContext,
    DependencyScope,
    DependencyContextId,
    DependencyContext
};

#[async_trait_with_sync::async_trait(Sync)]
pub (crate) trait ICycledComponentBuilder where Self: Sync + Send + 'static {
    async fn build(&self, ctx: Arc<DependencyCoreContext>, scope: Arc<DependencyScope>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>;
    fn get_input_type_info(&self) -> TypeInfo;
    fn get_output_type_info(&self) -> TypeInfo;
}

impl Debug for dyn ICycledComponentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CycledComponentBuilder")
            .field("[META]:input_type", &self.get_input_type_info())
            .field("[META]:output_ype", &self.get_output_type_info())
            .finish()
    }
}

// ----------- Maby move to self file -----------

#[derive(Default, new)]
pub (crate) struct SingletonComponentBuilder<TComponent: Sync + Send + 'static> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
}

#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for SingletonComponentBuilder<TComponent> {
    async fn build(&self, ctx: Arc<DependencyCoreContext>, scope: Arc<DependencyScope>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let singleton_component_type_id = TypeId::of::<Arc<TComponent>>();
        let global_scope_read_guard = scope.global_scope.read().await;
    
        if let Some(singleton_component_instance) = global_scope_read_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type type_id_expected:[{type_id:?}] type_name_expected:[{type_name:?}]",
                    type_id = &singleton_component_type_id,
                    type_name = type_name::<Arc<TComponent>>().to_string()
                ));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        drop(global_scope_read_guard); // дедлок!!!!!!!!!!!!!!!!!!!
        let mut global_scope_write_guard = scope.global_scope.write().await;
    
        if let Some(singleton_component_instance) = global_scope_write_guard.singletons.get(&singleton_component_type_id) {
            let singleton_component_instance: Arc<TComponent> = singleton_component_instance.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect singleton type type_id_expected:[{type_id:?}] type_name_expected:[{type_name:?}]",
                    type_id = &singleton_component_type_id,
                    type_name = type_name::<Arc<TComponent>>().to_string()
                ));
            return Ok(Box::new(singleton_component_instance) as Box<dyn Any + Sync + Send>);
        }
    
        let new_singleton = Arc::new(RwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        global_scope_write_guard.singletons.insert(singleton_component_type_id, new_singleton.clone());
    
        let mut new_singleton_write_guard = new_singleton.write().await;
        drop(global_scope_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ
    
        let component_type_id = TypeId::of::<TComponent>();
    
        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, ctx.clone(), scope.clone());
    
        let dependency = ctx.components.read().await.get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency TypeId:[{component_type_id:?}] type_name:[{type_name}]",
                type_name = type_name::<TComponent>().to_string()
            ))
            .clone();
    
        let new_component_instance = dependency.di_type.ctor.ctor(dependency_context).await?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_id:[{component_type_id:?}] type_name:[{type_name:?}] find type_id:[{expected_type_id:?}] type_name:[{expected_type_name:?}]",
                type_name = type_name::<TComponent>().to_string(),
                expected_type_id = dependency.di_type.id,
                expected_type_name = dependency.di_type.name
            ));
    
        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_singleton_write_guard.insert(new_component_instance_ref.clone());
    
            //global_scope_write_guard.singletons.insert(singleton_component_type_id, new_component_instance_ref.clone());

        return Ok(Box::new(new_component_instance_ref) as Box<dyn Any + Sync + Send>);
    }

    fn get_input_type_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }

    fn get_output_type_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<Arc<TComponent>>(), type_name::<Arc<TComponent>>().to_string())
    }
}

// ----------- Maby move to self file -----------

#[derive(Default, new)]
pub (crate) struct ScopedComponentBuilder<TComponent: Sync + Send + 'static> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
}

#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for ScopedComponentBuilder<TComponent> {
    async fn build(&self, ctx: Arc<DependencyCoreContext>, scope: Arc<DependencyScope>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let scoped_component_type_id = TypeId::of::<Arc<TComponent>>();
        let local_scope_read_guard = scope.local_scope.read().await;

        if let Some(scoped_component_instance_ref) = local_scope_read_guard.get(&scoped_component_type_id) {
            let scoped_component_instance: Arc<TComponent> = scoped_component_instance_ref.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect scoped type type_id_expected:[{type_id:?}] type_name_expected:[{type_name:?}]",
                    type_id = &scoped_component_type_id,
                    type_name = type_name::<Arc<TComponent>>().to_string()
                ));
            return Ok(Box::new(Arc::downgrade(&scoped_component_instance)) as Box<dyn Any + Sync + Send>);
        }

        drop(local_scope_read_guard);
        let mut local_scope_write_guard = scope.local_scope.write().await;

        if let Some(scoped_component_instance_ref) = local_scope_write_guard.get(&scoped_component_type_id) {
            let scoped_component_instance: Arc<TComponent> = scoped_component_instance_ref.read().await.as_ref().unwrap().clone().downcast::<TComponent>()
                .expect(&format!("Incorrect scoped type type_id_expected:[{type_id:?}] type_name_expected:[{type_name:?}]",
                    type_id = &scoped_component_type_id,
                    type_name = type_name::<Arc<TComponent>>().to_string()
                ));

            return Ok(Box::new(scoped_component_instance) as Box<dyn Any + Sync + Send>);
        }

        let new_scoped = Arc::new(RwLock::new(Option::<Arc<dyn Any + Sync + Send>>::None));
        local_scope_write_guard.insert(scoped_component_type_id, new_scoped.clone());

        let mut new_scoped_write_guard = new_scoped.write().await;
        drop(local_scope_write_guard); // Выглядит всрато, но надо отпустить лок всей коллекции, чтобы в дочерних элементах получить в кей доступ

        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, ctx.clone(), scope.clone());

        let dependency = ctx.components.read().await.get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency TypeId:[{component_type_id:?}] type_name:[{type_name}]",
                type_name = type_name::<TComponent>().to_string()
            ))
            .clone();

        let new_component_instance = dependency.di_type.ctor.ctor(dependency_context).await?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_id:[{component_type_id:?}] type_name:[{type_name:?}] find type_id:[{expected_type_id:?}] type_name:[{expected_type_name:?}]",
                type_name = type_name::<TComponent>().to_string(),
                expected_type_id = dependency.di_type.id,
                expected_type_name = dependency.di_type.name
            ));

        let new_component_instance_ref = Arc::new(Box::into_inner(new_component_instance));
        _ = new_scoped_write_guard.insert(new_component_instance_ref.clone());

            //local_scope_write_guard.insert(scoped_component_type_id, new_component_instance_ref.clone());

        Ok(Box::new(Arc::downgrade(&new_component_instance_ref)) as Box<dyn Any + Sync + Send>)
    }

    fn get_input_type_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }

    fn get_output_type_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<Weak<TComponent>>(), type_name::<Weak<TComponent>>().to_string())
    }
}

// ----------- Maby move to self file -----------

#[derive(Default, new)]
pub (crate) struct TransientComponentBuilder<TComponent: Sync + Send + 'static> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
}

#[async_trait_with_sync::async_trait(Sync)]
impl<TComponent: Sync + Send + 'static> ICycledComponentBuilder for TransientComponentBuilder<TComponent> {
    async fn build(&self, ctx: Arc<DependencyCoreContext>, scope: Arc<DependencyScope>) -> BuildDependencyResult<Box<dyn Any + Sync + Send>>{
        let component_type_id = TypeId::of::<TComponent>();

        let dependency_context_id = DependencyContextId::TypeId(TypeInfo::from_type::<TComponent>());
        let dependency_context = DependencyContext::new_dependency(dependency_context_id, ctx.clone(), scope.clone());

        let dependency = ctx.components.read().await.get(&component_type_id)
            .expect(&format!("dependency not found, expected checked dependency TypeId:[{component_type_id:?}] type_name:[{type_name}]",
                type_name = type_name::<TComponent>().to_string()
            ))
            .clone();

        let new_component_instance = dependency.di_type.ctor.ctor(dependency_context).await?;
        let new_component_instance: Box<TComponent> = new_component_instance.downcast::<TComponent>()
            .expect(&format!("expected type_id:[{component_type_id:?}] type_name:[{type_name:?}] find type_id:[{expected_type_id:?}] type_name:[{expected_type_name:?}]",
                type_name = type_name::<TComponent>().to_string(),
                expected_type_id = dependency.di_type.id,
                expected_type_name = dependency.di_type.name
            ));

        Ok(new_component_instance as Box<dyn Any + Sync + Send>)
    }

    fn get_input_type_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }

    fn get_output_type_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }
}