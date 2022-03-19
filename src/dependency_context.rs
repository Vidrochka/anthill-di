use crate::{DependencyLink, types::BuildDependencyError};
use std::{
    any::{TypeId, type_name},
    sync::{
        Arc,
        Weak
    }
};

use crate::{
    DependencyCoreContext,
    DependencyScope,
    DependencyBuilder,
    Dependency,
    types::{BuildDependencyResult, AddDependencyResult, AddDependencyError},
    TypeConstructor, DependencyLifeCycle, DependencyType, base::{SingletonConstructor, ScopedConstructor}
};

#[derive(PartialEq, Clone)]
pub (crate) enum DependencyContextId {
    TypeId(TypeId, String),
    Root,
}

pub struct DependencyContext {
    id: DependencyContextId,
    ctx: Arc<DependencyCoreContext>,
    pub (crate) scope: Arc<DependencyScope>,
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
        let dependency_type = DependencyType::new::<TType>(ctor);
        let dependency = Dependency::new(DependencyLifeCycle::Transient, dependency_type);

        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;
        let mut dependency_links_guard = self.ctx.dependency_link_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone()});
        }

        dependency_links_guard.insert(dependency.di_type.id.clone(), DependencyLink::new());
        dependency_collection_guard.insert(dependency.di_type.id.clone(), Arc::new(dependency));

        Ok(())
    }

    pub async fn add_singleton<TType: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<()> {
        let ctor = Box::new(SingletonConstructor::new::<TType>(ctor));
        let dependency_type = DependencyType::new::<Arc<TType>>(ctor);
        let dependency = Dependency::new(DependencyLifeCycle::Singleton, dependency_type);

        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;
        let mut dependency_links_guard = self.ctx.dependency_link_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone()});
        }

        dependency_links_guard.insert(dependency.di_type.id.clone(), DependencyLink::new());
        dependency_collection_guard.insert(dependency.di_type.id.clone(), Arc::new(dependency));
        
        Ok(())
    }

    pub async fn add_scoped<TType: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<()> {
        let ctor = Box::new(ScopedConstructor::new::<TType>(ctor));
        let dependency_type = DependencyType::new::<Weak<TType>>(ctor);
        let dependency = Dependency::new(DependencyLifeCycle::Scoped, dependency_type);

        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;
        let mut dependency_links_guard = self.ctx.dependency_link_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone() });
        }

        dependency_links_guard.insert(dependency.di_type.id.clone(), DependencyLink::new());
        dependency_collection_guard.insert(dependency.di_type.id.clone(), Arc::new(dependency));

        Ok(())
    }

    pub async fn add_singleton_instance<TType: Sync + Send + 'static>(&self, instance: TType) -> AddDependencyResult<()> {
        let ctor = Box::new(SingletonConstructor::new_with_instance(Arc::new(instance)));
        let dependency_type = DependencyType::new::<Arc<TType>>(ctor);
        let dependency = Dependency::new(DependencyLifeCycle::Singleton, dependency_type);

        let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;
        let mut dependency_links_guard = self.ctx.dependency_link_collection.write().await;

        if dependency_collection_guard.contains_key(&dependency.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone()});
        }

        dependency_links_guard.insert(dependency.di_type.id.clone(), DependencyLink::new());
        dependency_collection_guard.insert(dependency.di_type.id.clone(), Arc::new(dependency));

        Ok(())
    }

    // Check link tree and build dependency
    pub async fn get<TType: Sync + Send + 'static>(&self) -> BuildDependencyResult<TType> {
        if !self.ctx.dependency_collection.read().await.contains_key(&TypeId::of::<TType>()) {
            return Err(BuildDependencyError::NotFound{
                id: TypeId::of::<TType>(),
                name: type_name::<TType>().to_string(),
            })
        }

        if let DependencyContextId::TypeId(type_id, parent_name) = &self.id {
            DependencyBuilder::try_add_link::<TType>(self.ctx.clone(), type_id, parent_name).await?;
        }
       
        DependencyBuilder::build(self.scope.clone(), self.ctx.clone()).await
    }
}