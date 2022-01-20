use std::{
    any::{TypeId, type_name, Any},
    sync::{
        Arc,
        Weak
    }
};

use tokio::sync::RwLock;

use crate::{
    DependencyCoreContext,
    DependencyScope,
    DependencyBuilder,
    Dependency,
    types::{BuildDependencyResult, AddDependencyResult, AddDependencyError},
    TypeConstructor
};

#[derive(PartialEq, Clone)]
pub (crate) enum DependencyContextId {
    TypeId(TypeId),
    Root,
}

pub struct DependencyContext {
    id: DependencyContextId,
    ctx: Arc<DependencyCoreContext>,
    scope: Arc<DependencyScope>,
}

impl DependencyContext {
    pub fn new_root() -> Self {
        Self {
            id: DependencyContextId::Root,
            ctx: Arc::new(DependencyCoreContext::new()),
            scope: Arc::new(DependencyScope::new()),
        }
    }

    pub (crate) fn new_dependency(id: DependencyContextId, ctx: Arc<DependencyCoreContext>, scope: Arc<DependencyScope>) -> Self {
        Self {
            id,
            ctx,
            scope,
        }
    }

    pub fn set_scope(&mut self, scope: Arc<DependencyScope>) { self.scope = scope }
    pub fn set_empty_scope(&mut self) -> Arc<DependencyScope> {
        self.scope = Arc::new(DependencyScope::new());
        self.scope.clone() 
    }
    pub fn get_scope(&self) -> Arc<DependencyScope> { self.scope.clone() }

    pub async fn add_transient<TType: 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<()> {
        let dependency = Dependency::new_transient::<TType>(ctor);
        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: type_name::<TType>().to_string()});
        }

        dependency_collection_guard.insert(dependency.di_type.id, Arc::new(dependency));
        Ok(())
    }

    pub async fn add_singleton<TType: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<()> {
        let dependency = Dependency::new_singleton::<TType>(ctor);
        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: type_name::<TType>().to_string()});
        }

        dependency_collection_guard.insert(dependency.di_type.id, Arc::new(dependency));
        Ok(())
    }

    pub async fn add_scoped<TType: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<()> {
        let dependency = Dependency::new_scoped::<TType>(ctor);
        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: type_name::<TType>().to_string()});
        }

        dependency_collection_guard.insert(dependency.di_type.id, Arc::new(dependency));
        Ok(())
    }

    pub async fn add_singleton_instance<TType: Sync + Send + 'static>(&self, instance: TType) -> AddDependencyResult<()> {
        let id = TypeId::of::<TType>();
        let mut singleton_dependency_guard = self.ctx.singleton_dependency.write().await;

        if singleton_dependency_guard.contains_key(&id) {
            return Err(AddDependencyError::DependencyExist { id, name: type_name::<TType>().to_string() });
        }

        let new_singleton = Arc::new(RwLock::new(Some(Arc::new(instance) as Arc<dyn Any + Sync + Send>)));
        singleton_dependency_guard.insert(id, new_singleton);

        Ok(())
    }

    pub async fn add_scoped_instance<TType: Sync + Send + 'static>(&self, instance: TType) -> AddDependencyResult<()> {
        let id = TypeId::of::<TType>();
        let mut scoped_dependency_guard = self.scope.scoped_dependencies.write().await;

        if scoped_dependency_guard.contains_key(&id) {
            return Err(AddDependencyError::DependencyExist { id, name: type_name::<TType>().to_string() });
        }

        let new_scoped = Arc::new(RwLock::new(Some(Arc::new(instance) as Arc<dyn Any + Sync + Send>)));
        scoped_dependency_guard.insert(id, new_scoped);

        Ok(())
    }

    pub async fn get_transient<TType: 'static>(&self) -> BuildDependencyResult<TType> {
        DependencyBuilder::try_add_link::<TType>(self.ctx.clone(), self.id.clone()).await?;
        DependencyBuilder::build_transient(self.scope.clone(), self.ctx.clone()).await
    }

    pub async fn get_singleton<TType: Sync + Send + 'static>(&self) -> BuildDependencyResult<Arc<TType>> {
        DependencyBuilder::try_add_link::<TType>(self.ctx.clone(), self.id.clone()).await?;
        DependencyBuilder::build_singleton(self.scope.clone(), self.ctx.clone()).await
    }

    pub async fn get_scoped<TType: Sync + Send + 'static>(&self) -> BuildDependencyResult<Weak<TType>> {
        DependencyBuilder::try_add_link::<TType>(self.ctx.clone(), self.id.clone()).await?;
        DependencyBuilder::build_scoped(self.scope.clone(), self.ctx.clone()).await
    }
}