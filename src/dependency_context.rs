use tokio::sync::RwLock;
use crate::{Constructor, ComponentWithConstructor, SingletonComponentBuilder, ScopedComponentBuilder, TransientComponentBuilder, ICycledComponentBuilder};
use std::marker::Unsize;
use crate::{DependencyLink, types::{BuildDependencyError, MapComponentResult, MapComponentError}, ServiceMappingsCollection, NoLogicService};
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
    TypeConstructor,
    DependencyLifeCycle,
    DependencyType,
    // base::{
    //     SingletonConstructor,
    //     ScopedConstructor
    // }
};

#[derive(Debug, PartialEq, Clone)]
pub (crate) enum DependencyContextId {
    TypeId(TypeId, String),
    Root,
}

#[derive(Debug)]
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

    pub async fn register<TComponent: Constructor + Sync + Send + 'static>(&self, life_cycle: DependencyLifeCycle) -> AddDependencyResult<&Self> {
        let component_type = DependencyType::new::<TComponent>(Box::new(ComponentWithConstructor::<TComponent>::new()));
        let component = Dependency::new(life_cycle.clone(), component_type);

        let component_id = component.di_type.id.clone();

        // Проверяем наличие зависимости, если нет добавляем
        let mut components_guard = self.ctx.components.write().await;
        
        if components_guard.contains_key(&component.di_type.id) {
            return Err(AddDependencyError::DependencyExist { id: component_id, name: component.di_type.name.clone()});
        }
   
        components_guard.insert(component_id.clone(), Arc::new(component));
        //---------------------------

        // Создаем ячейку свзяей без связей //TODO: в линках брать реальный ид
        let mut links_guard = self.ctx.links.write().await;
        links_guard.insert(component_id.clone(), DependencyLink::new());
        //---------------------------

        // Пустой маппинг сомпонента
        let mut services_write_guard = self.ctx.services.write().await;

        let typed_component_id = match life_cycle {
            DependencyLifeCycle::Transient => TypeId::of::<TComponent>(),
            DependencyLifeCycle::Singleton => TypeId::of::<Arc<TComponent>>(),
            DependencyLifeCycle::Scoped => TypeId::of::<Weak<TComponent>>(),
        };

        if !services_write_guard.contains_key(&typed_component_id) {
            let service_mapping_collection = match life_cycle {
                DependencyLifeCycle::Transient => ServiceMappingsCollection::new::<TComponent>(),
                DependencyLifeCycle::Singleton =>  ServiceMappingsCollection::new::<Arc<TComponent>>(),
                DependencyLifeCycle::Scoped => ServiceMappingsCollection::new::<Weak<TComponent>>(),
            };

            services_write_guard.insert(service_mapping_collection.get_service_info().type_id, Arc::new(RwLock::new(service_mapping_collection)));
        }

        match life_cycle {
            DependencyLifeCycle::Transient => {
                let services = services_write_guard.get_mut(&typed_component_id).unwrap().clone();
                services.write().await.add_mapping_component_to_component::<TComponent>();
            },
            DependencyLifeCycle::Singleton => {
                let services = services_write_guard.get_mut(&typed_component_id).unwrap().clone();
                services.write().await.add_mapping_component_to_component::<Arc<TComponent>>();
            },
            DependencyLifeCycle::Scoped => {
                let services = services_write_guard.get_mut(&typed_component_id).unwrap().clone();
                services.write().await.add_mapping_component_to_component::<Weak<TComponent>>();
            },
        }
        //---------------------------

        // Регистрируем обработчик лайфтайма компонента
        let mut cycle_builders_write_guard = self.ctx.cycled_component_builders.write().await;

        let cycled_component_builder: Box<dyn ICycledComponentBuilder> = match life_cycle {
            DependencyLifeCycle::Transient => Box::new(TransientComponentBuilder::<TComponent>::new()),
            DependencyLifeCycle::Singleton => Box::new(SingletonComponentBuilder::<TComponent>::new()),
            DependencyLifeCycle::Scoped => Box::new(ScopedComponentBuilder::<TComponent>::new()),
        };

        cycle_builders_write_guard.insert(cycled_component_builder.get_output_type_info().type_id, Arc::new(cycled_component_builder));

        //---------------------------

        Ok(self)
    }

    // pub async fn add_transient<TComponent: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<&Self> {
    //     let dependency_type = DependencyType::new::<TComponent>(ctor);
    //     let dependency = Dependency::new(DependencyLifeCycle::Transient, dependency_type);

    //     let component_id = dependency.di_type.id.clone();

    //     // Проверяем наличие зависимости, если нет добавляем
    //     let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;
        
    //     if dependency_collection_guard.contains_key(&dependency.di_type.id) {
    //         return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone()});
    //     }
   
    //     dependency_collection_guard.insert(component_id.clone(), Arc::new(dependency));
    //     //---------------------------

    //     // Создаем ячейку свзяей без связей
    //     let mut dependency_links_guard = self.ctx.dependency_link_collection.write().await;
    //     dependency_links_guard.insert(component_id.clone(), DependencyLink::new());
    //     //---------------------------

    //     // Пустой маппинг сомпонента
    //     let mut services_write_guard = self.ctx.services.write().await;

    //     if !services_write_guard.contains_key(&component_id) {
    //         services_write_guard.insert(component_id, ServiceMappingsCollection::new());
    //     }

    //     let services = services_write_guard.get_mut(&dependency.di_type.id).expect("We check services exist, wtf???");
    //     services.add_mapping_component_to_component::<TComponent>();
    //     //---------------------------

    //     Ok(self)
    // }

    // pub async fn add_singleton<TType: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<&Self> {
    //     let ctor = Box::new(SingletonConstructor::new::<TType>(ctor));
    //     let dependency_type = DependencyType::new::<Arc<TType>>(ctor);
    //     let dependency = Dependency::new(DependencyLifeCycle::Singleton, dependency_type);

    //     let mut dependency_collection_guard = self.ctx.components.write().await;
    //     let mut dependency_links_guard = self.ctx.links.write().await;

    //     if dependency_collection_guard.contains_key(&dependency.di_type.id) {
    //         return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone()});
    //     }

    //     dependency_links_guard.insert(dependency.di_type.id.clone(), DependencyLink::new());
    //     dependency_collection_guard.insert(dependency.di_type.id.clone(), Arc::new(dependency));
        
    //     Ok(self)
    // }

    // pub async fn add_scoped<TType: Sync + Send + 'static>(&self, ctor: Box<dyn TypeConstructor>) -> AddDependencyResult<&Self> {
    //     let ctor = Box::new(ScopedConstructor::new::<TType>(ctor));
    //     let dependency_type = DependencyType::new::<Weak<TType>>(ctor);
    //     let component = Dependency::new(DependencyLifeCycle::Scoped, dependency_type);

    //     let mut components_guard = self.ctx.components.write().await;
    //     let mut links_guard = self.ctx.links.write().await;

    //     if components_guard.contains_key(&component.di_type.id) {
    //         return Err(AddDependencyError::DependencyExist { id: component.di_type.id.clone(), name: component.di_type.name.clone() });
    //     }

    //     links_guard.insert(component.di_type.id.clone(), DependencyLink::new());
    //     components_guard.insert(component.di_type.id.clone(), Arc::new(component));

    //     Ok(self)
    // }

    // pub async fn add_singleton_instance<TType: Sync + Send + 'static>(&self, instance: TType) -> AddDependencyResult<&Self> {
    //     let ctor = Box::new(SingletonConstructor::new_with_instance(Arc::new(instance)));
    //     let dependency_type = DependencyType::new::<Arc<TType>>(ctor);
    //     let dependency = Dependency::new(DependencyLifeCycle::Singleton, dependency_type);

    //     let mut dependency_collection_guard = self.ctx.dependency_collection.write().await;
    //     let mut dependency_links_guard = self.ctx.dependency_link_collection.write().await;

    //     if dependency_collection_guard.contains_key(&dependency.di_type.id) {
    //         return Err(AddDependencyError::DependencyExist { id: dependency.di_type.id.clone(), name: dependency.di_type.name.clone()});
    //     }

    //     dependency_links_guard.insert(dependency.di_type.id.clone(), DependencyLink::new());
    //     dependency_collection_guard.insert(dependency.di_type.id.clone(), Arc::new(dependency));

    //     Ok(self)
    // }

    pub async fn map_component_as_trait_service<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&self) -> MapComponentResult<&Self> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();
        let service_id = TypeId::of::<Box<TService>>();

        if !self.ctx.cycled_component_builders.read().await.contains_key(&component_id) {
            return Err(MapComponentError::ComponentNotFound{
                id: component_id.clone(),
                name: type_name::<TComponent>().to_string(),
            });
        }

        let mut services_write_lock = self.ctx.services.write().await;

        if !services_write_lock.contains_key(&service_id) {
            let service_mappings_collection = ServiceMappingsCollection::new::<Box<TService>>();
            services_write_lock.insert(service_mappings_collection.get_service_info().type_id, Arc::new(RwLock::new(service_mappings_collection)));
        }

        let services = services_write_lock.get_mut(&service_id).unwrap().clone();        
        services.write().await.add_mapping_component_as_trait_service::<TComponent,TService>();

        Ok(self)
    }

    // Check link tree and build dependency
    pub async fn get<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<TService> {
        let service_id = TypeId::of::<TService>();

        let services = match self.ctx.services.read().await.get(&service_id) {
            Some(services) => services.clone(),
            None => return Err(BuildDependencyError::NotFound{
                id: service_id,
                name: type_name::<TService>().to_string(),
            }),
        };

        if let DependencyContextId::TypeId(type_id, parent_name) = &self.id {
            DependencyBuilder::try_add_link::<TService>(self.ctx.clone(), type_id, parent_name).await?;
        }

        let services_read_lock = services.read().await;

        // { cycled_component_type_id , service_constructor }
        let service_info = services_read_lock.get_nth_service_info(0);

        let cycled_component_builder = self.ctx.cycled_component_builders.read().await
            .get(&service_info.0)
            .expect(&format!("Service exist but cycled component builder not found service_id:[{service_id:?}]"))
            .clone();

        let cycled_component = cycled_component_builder.build(self.ctx.clone(), self.scope.clone()).await?;
        let service: Box<TService> = service_info.1.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}]", service_name = type_name::<TService>().to_string()));

        return Ok(Box::into_inner(service));
       
        //DependencyBuilder::build(self.scope.clone(), self.ctx.clone()).await
    }
}