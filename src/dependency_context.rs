use tokio::runtime::Builder;
use crate::{Constructor, ComponentFromConstructor, types::TypeInfo, constructors::{ComponentFromAsyncClosure, ComponentFromClosure, ComponentFromInstance}};
use std::{marker::Unsize, future::Future};
use std::{
    any::TypeId,
    sync::Arc,
};

#[cfg(feature = "loop-check")]
use crate::DependencyLink;

use crate::{
    DependencyCoreContext,
    DependencyScope,
    DependencyBuilder,
    types::{
        BuildDependencyResult,
        AddDependencyResult,
        MapComponentResult,
    },
    DependencyLifeCycle,
    DependencyType,
};

#[derive(Debug, PartialEq, Clone)]
pub (crate) enum DependencyContextId {
    TypeId(TypeInfo),
    Root,
}

#[derive(Debug, Clone)]
pub struct DependencyContext {
    id: DependencyContextId,
    ctx: Arc<DependencyCoreContext>,
    pub (crate) scope: Arc<DependencyScope>,
}

impl DependencyContext {
    pub fn new_root() -> Self {
        let context = DependencyCoreContext::new();
        let scope = DependencyScope::new(context.global_scope.clone());

        Self {
            id: DependencyContextId::Root,
            ctx: Arc::new(context),
            scope: Arc::new(scope),
        }
    }

    pub (crate) fn new_dependency(id: DependencyContextId, ctx: Arc<DependencyCoreContext>, scope: Arc<DependencyScope>) -> Self {
        Self { id, ctx, scope, }
    }

    pub fn set_scope(&mut self, scope: Arc<DependencyScope>) { self.scope = scope }
    pub fn set_empty_scope(&mut self) -> Arc<DependencyScope> {
        self.scope = Arc::new(DependencyScope::new(self.ctx.global_scope.clone()));
        self.scope.clone()
    }
    pub fn get_scope(&self) -> Arc<DependencyScope> { self.scope.clone() }

    pub async fn register_type<TComponent: Constructor + Sync + Send + 'static>(&self, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromConstructor::<TComponent>::new()));
        self.ctx.register::<TComponent>(component_type, life_cycle).await
    }

    pub fn register_type_sync<TComponent: Constructor + Sync + Send + 'static>(&self, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromConstructor::<TComponent>::new()));
        self.register_sync::<TComponent>(component_type, life_cycle)
    }

    pub async fn register_async_closure<TComponent, TFuture, TClosure>(&self, closure: TClosure, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>>
    where
        TComponent: Sync + Send + 'static,
        TFuture: Future<Output = BuildDependencyResult<TComponent>>,
        TFuture: Sync + Send + 'static,
        TClosure: Fn(DependencyContext) -> TFuture,
        TClosure: Sync + Send + 'static,
    {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromAsyncClosure::<TComponent, TFuture, TClosure>::new(closure)));
        self.ctx.register::<TComponent>(component_type, life_cycle).await
    }

    pub fn register_async_closure_sync<TComponent, TFuture, TClosure>(&self, closure: TClosure, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>>
    where
        TComponent: Sync + Send + 'static,
        TFuture: Future<Output = BuildDependencyResult<TComponent>>,
        TFuture: Sync + Send + 'static,
        TClosure: Fn(DependencyContext) -> TFuture,
        TClosure: Sync + Send + 'static,
    {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromAsyncClosure::<TComponent, TFuture, TClosure>::new(closure)));
        self.register_sync::<TComponent>(component_type, life_cycle)
    }

    pub async fn register_closure<TComponent: Sync + Send + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send + 'static>(&self, closure: TClosure, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromClosure::<TComponent>::new(Box::new(closure))));
        self.ctx.register::<TComponent>(component_type, life_cycle).await
    }

    pub fn register_closure_sync<TComponent: Sync + Send + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send + 'static>(&self, closure: TClosure, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromClosure::<TComponent>::new(Box::new(closure))));
        self.register_sync::<TComponent>(component_type, life_cycle)
    }

    pub async fn register_instance<TComponent: Sync + Send + 'static>(&self, instance: TComponent) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromInstance::new(instance)));
        self.ctx.register::<TComponent>(component_type, DependencyLifeCycle::Singleton).await
    }

    pub fn register_instance_sync<TComponent: Sync + Send + 'static>(&self, instance: TComponent) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentFromInstance::new(instance)));
        self.register_sync::<TComponent>(component_type, DependencyLifeCycle::Singleton)
    }

    pub (crate) fn register_sync<TComponent: Sync + Send + 'static>(&self, component_type: DependencyType, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let ctx = self.ctx.clone();

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async move { ctx.register::<TComponent>(component_type, life_cycle).await })
        }).join().unwrap()
    }

    pub async fn map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&self) -> MapComponentResult<Arc<DependencyCoreContext>> where TComponent: Unsize<TService> {
        self.ctx.map_component::<TComponent, TService>().await
    }

    pub fn map_component_sync<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&self) -> MapComponentResult<Arc<DependencyCoreContext>> where TComponent: Unsize<TService> {
        let ctx = self.ctx.clone();

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async { ctx.map_component::<TComponent,TService>().await })
        }).join().unwrap()
    }

    /// Resolve first (by TypeId) dependency
    pub async fn resolve<'a, TService: Sync + Send + 'static>(&'a self) -> BuildDependencyResult<TService> {
        self.ctx.resolve::<TService>(self.id.clone(), self.scope.clone()).await
    }

    pub fn resolve_sync<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<TService> {
        let ctx = self.ctx.clone();
        let id = self.id.clone();
        let scope = self.scope.clone();

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async { ctx.resolve::<TService>(id, scope).await })
        }).join().unwrap()
    }

    pub async fn resolve_by_type_id<TService: Sync + Send + 'static>(&self, component_type_id: TypeId) -> BuildDependencyResult<TService> {
        self.ctx.resolve_by_type_id::<TService>(component_type_id, self.id.clone(), self.scope.clone()).await
    }

    pub fn resolve_by_type_id_sync<TService: Sync + Send + 'static>(&self, component_type_id: TypeId) -> BuildDependencyResult<TService> {
        let ctx = self.ctx.clone();
        let id = self.id.clone();
        let scope = self.scope.clone();

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async { ctx.resolve_by_type_id::<TService>(component_type_id, id, scope).await })
        }).join().unwrap()
    }

    pub async fn resolve_collection<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<Vec<TService>> {
        self.ctx.resolve_collection::<TService>(self.id.clone(), self.scope.clone()).await
    }

    pub fn resolve_collection_sync<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<Vec<TService>> {
        let ctx = self.ctx.clone();
        let id = self.id.clone();
        let scope = self.scope.clone();

        std::thread::spawn(move || {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async { ctx.resolve_collection(id, scope).await })
        }).join().unwrap()
    }
}