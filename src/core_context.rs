use std::{
    collections::{HashMap, VecDeque},
    any::{TypeId, type_name},
    sync::{Arc, Weak},
    fmt::Debug, marker::Unsize
};

#[cfg(feature = "loop-check")]
use crate::DependencyLink;

use crate::{
    Component,
    service::{
        CycledComponentServiceCollection,
    },
    cycled_components::ComponentCycledComponentCollection,
    GlobalContext,
    LifeCycle,
    types::{
        AddDependencyResult,
        TypeInfo,
        AddDependencyError,
        BuildDependencyResult,
        BuildDependencyError, 
        MapComponentError,
        MapComponentResult,
        DeleteComponentResult,
        DeleteComponentError,
        AnthillRwLock,
    },
    ServiceMappingBuilder,
    DependencyContextId,
    LocalContext,
    component::ITypeConstructor
};

#[derive(Default)]
pub struct CoreContext where Self: Sync + Send {
    pub (crate) components: AnthillRwLock<HashMap<TypeId, Arc<Component>>>,
    pub (crate) component_cycled_components_collection: AnthillRwLock<ComponentCycledComponentCollection>,
    pub (crate) cycled_component_service_collection: AnthillRwLock<CycledComponentServiceCollection>,

    pub (crate) global_context: Arc<AnthillRwLock<GlobalContext>>,
    #[cfg(feature = "loop-check")]
    pub (crate) links: AnthillRwLock<HashMap<TypeId, DependencyLink>>,
}

impl CoreContext {
    pub fn new() -> Self { 
        Self {
            components: Default::default(),
            component_cycled_components_collection: Default::default(),
            cycled_component_service_collection: Default::default(),
            global_context: Default::default(),
            #[cfg(feature = "loop-check")]
            links: Default::default(),
        }
    }
}

impl Debug for CoreContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("DependencyCoreContext");
        debug_struct.field("component_cycled_components_collection", &self.component_cycled_components_collection.try_read().unwrap())
            .field("components", &self.components.try_read().unwrap())
            .field("cycled_component_service_collection", &self.cycled_component_service_collection.try_read().unwrap());

        #[cfg(feature = "loop-check")]
        debug_struct.field("links", &self.links.try_read().unwrap());

        debug_struct.field("global_context", &self.global_context.try_read().unwrap())
            .finish()
    }
}

#[cfg(feature = "async-mode")]
impl CoreContext {
    pub (crate) async fn register<TComponent: Sync + Send + 'static>(self: &Arc<Self>, ctor: Box<dyn ITypeConstructor>, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        let component = Component::new::<TComponent>(life_cycle.clone(), ctor);

        let component_id = component.component_type_info.type_id.clone();

        // Проверяем наличие зависимости, если нет добавляем
        let mut components_guard = self.components.write().await;
        
        if components_guard.contains_key(&component.component_type_info.type_id) {
            return Err(AddDependencyError::DependencyExist { component_type_info: TypeInfo::from_type::<TComponent>() });
        }
   
        components_guard.insert(component_id.clone(), Arc::new(component));
        //---------------------------

        // Создаем ячейку свзяей без связей //TODO: в линках брать реальный ид
        #[cfg(feature = "loop-check")]
        self.links.write().await.insert(component_id, DependencyLink::new());
        //---------------------------

        // Пустой маппинг сомпонента
        match life_cycle {
            LifeCycle::Transient => self.cycled_component_service_collection.write().await.add_mapping_as_self::<TComponent>(),
            LifeCycle::Singleton => self.cycled_component_service_collection.write().await.add_mapping_as_self::<Arc<TComponent>>(),
            LifeCycle::ContextDependent => self.cycled_component_service_collection.write().await.add_mapping_as_self::<Weak<TComponent>>(),
        }
        //---------------------------

        // Регистрируем обработчик лайфтайма компонента
        match life_cycle {
            LifeCycle::Transient => self.component_cycled_components_collection.write().await.add_transient_cycle_builder::<TComponent>(),
            LifeCycle::Singleton => self.component_cycled_components_collection.write().await.add_singleton_cycle_builder::<TComponent>(),
            LifeCycle::ContextDependent => self.component_cycled_components_collection.write().await.add_context_dependent_cycle_builder::<TComponent>(),
        };
        //---------------------------

        Ok(ServiceMappingBuilder::new(self.clone()))
    }

    pub (crate) async fn resolve<'a, TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<TService>{//std::pin::Pin<Box<dyn std::future::Future<Output = BuildDependencyResult<TService>> + Send + Sync + 'a>> {
        let service_id = TypeId::of::<TService>();

        let component_service_pair = self.cycled_component_service_collection.read().await.get_nth_by_service_type::<TService>(0)
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;
    
        let component_cycled_component_pair = self.component_cycled_components_collection.read().await
            .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
            .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));
        
        #[cfg(feature = "loop-check")]
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info).await?;
        }
    
        let cycled_component = component_cycled_component_pair.converter.build(self.clone(), local_context).await?;
        let service: Box<TService> = component_service_pair.converter.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

        Ok(Box::into_inner(service))
    }

    pub (crate) async fn resolve_by_type_id<TService: Sync + Send + 'static>(self: &Arc<Self>, component_type_id: TypeId, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<TService> {
        let service_id = TypeId::of::<TService>();

        let component_service_pair = self.cycled_component_service_collection.read().await.get_all_by_service_type_with_cycled_component_id::<TService>(component_type_id)
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let component_cycled_component_pair = self.component_cycled_components_collection.read().await
            .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
            .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));

        #[cfg(feature = "loop-check")]
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info).await?;
        }

        let cycled_component = component_cycled_component_pair.converter.build(self.clone(), local_context).await?;
        let service: Box<TService> = component_service_pair.converter.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

        return Ok(Box::into_inner(service));
    }

    pub (crate) async fn resolve_collection<TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<Vec<TService>> {
        let service_id = TypeId::of::<TService>();

        let component_service_pairs = self.cycled_component_service_collection.read().await.get_all_by_service_type::<TService>()
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let mut result = Vec::new();
        for component_service_pair in component_service_pairs.iter() {
            let component_cycled_component_pair = self.component_cycled_components_collection.read().await
                .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
                .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));

            #[cfg(feature = "loop-check")]
            if let DependencyContextId::TypeId(type_info) = &id {
                // Link created on dependency add, we need take link for dependency, not cycled dependency or service
                check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info).await?;
            }

            let cycled_component = component_cycled_component_pair.converter.build(self.clone(), local_context.clone()).await?;
            let service: Box<TService> = component_service_pair.converter.build(cycled_component)
                .downcast::<TService>()
                .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

            result.push(Box::into_inner(service));
        }

       return Ok(result);
    }

    pub async fn map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(self: &Arc<Self>) -> MapComponentResult<Arc<Self>> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.components.read().await;
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{
                component_type_info: TypeInfo::from_type::<TComponent>(),
                service_type_info: TypeInfo::from_type::<TService>(),
            });
        }

        let component = component.unwrap();

        match component.life_cycle_type {
            LifeCycle::Transient => self.cycled_component_service_collection.write().await.add_mapping_as_transient::<TComponent, TService>(),
            LifeCycle::Singleton => self.cycled_component_service_collection.write().await.add_mapping_as_singleton::<TComponent, TService>(),
            LifeCycle::ContextDependent => self.cycled_component_service_collection.write().await.add_mapping_as_context_dependent::<TComponent, TService>(),
        };

        Ok(self.clone())
    }

    pub async fn delete_component<TComponent: 'static>(&self) -> DeleteComponentResult<()> {
        // Get cycled_component_service_collection write guard at first, because resolve service start from cycled_component_service_collection read. Here we block resolve before delete
        // I dont make top RwLock, because in resolve time we can add new dependency from ctr, that mean write lock and then deadlock, because top level RwLock will be in read lock
        let mut cycled_component_service_collection_write_guard = self.cycled_component_service_collection.write().await;

        // We check life cycle & check component existence in one time
        let life_cycle = self.components.read().await.get(&TypeId::of::<TComponent>())
            .ok_or(DeleteComponentError::ComponentNotFound { component_type_info: TypeInfo::from_type::<TComponent>() })?.life_cycle_type;

        if ![LifeCycle::Singleton, LifeCycle::Transient].contains(&life_cycle) {
            return Err(DeleteComponentError::NotSupportedLifeCycle { component_type_info: TypeInfo::from_type::<TComponent>(), life_cycle })
        }

        let component = self.components.write().await
            .remove(&TypeId::of::<TComponent>())
            .unwrap();

        #[cfg(feature = "loop-check")]
        let _ = self.links.write().await.remove(&TypeId::of::<TComponent>()).unwrap();

        let cycled_component_builder = self.component_cycled_components_collection.write().await
            .delete_by_component::<TComponent>()
            .unwrap();

        match component.life_cycle_type {
            // Singleton mayby not exist, because not requested
            LifeCycle::Singleton => _ = self.global_context.write().await.singletons
                    .remove(&cycled_component_builder.cycled_component_type_info.type_id),
            LifeCycle::ContextDependent => panic!("Failed life cycle check before delete, life cycle:[{life_cycle:?}]"),
            _ => {}
        }

        _ = cycled_component_service_collection_write_guard.delete_cycled_component(&cycled_component_builder.cycled_component_type_info.type_id)
            .unwrap();
        
        Ok(())
    }

    #[inline(always)]
    pub (crate) async fn is_service_exist(&self, service_type_id: TypeId) -> bool {
        self.cycled_component_service_collection.read().await.is_service_exist(&service_type_id)
    }

    #[inline(always)]
    pub (crate) async fn is_component_exist(&self, component_type_id: TypeId) -> bool {
        self.components.read().await.contains_key(&component_type_id)
    }
}

#[cfg(feature = "blocking")]
impl CoreContext {
    pub (crate) fn blocking_register<TComponent: Sync + Send + 'static>(self: &Arc<Self>, ctor: Box<dyn ITypeConstructor>, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        let component = Component::new::<TComponent>(life_cycle.clone(), ctor);

        let component_id = component.component_type_info.type_id.clone();

        // Проверяем наличие зависимости, если нет добавляем
        let mut components_guard = self.components.blocking_write();
        
        if components_guard.contains_key(&component.component_type_info.type_id) {
            return Err(AddDependencyError::DependencyExist { component_type_info: TypeInfo::from_type::<TComponent>() });
        }
   
        components_guard.insert(component_id.clone(), Arc::new(component));
        //---------------------------

        // Создаем ячейку свзяей без связей //TODO: в линках брать реальный ид
        #[cfg(feature = "loop-check")]
        self.links.blocking_write().insert(component_id, DependencyLink::new());
        //---------------------------

        // Пустой маппинг сомпонента
        match life_cycle {
            LifeCycle::Transient => self.cycled_component_service_collection.blocking_write().add_mapping_as_self::<TComponent>(),
            LifeCycle::Singleton => self.cycled_component_service_collection.blocking_write().add_mapping_as_self::<Arc<TComponent>>(),
            LifeCycle::ContextDependent => self.cycled_component_service_collection.blocking_write().add_mapping_as_self::<Weak<TComponent>>(),
        }
        //---------------------------

        // Регистрируем обработчик лайфтайма компонента
        match life_cycle {
            LifeCycle::Transient => self.component_cycled_components_collection.blocking_write().add_transient_cycle_builder::<TComponent>(),
            LifeCycle::Singleton => self.component_cycled_components_collection.blocking_write().add_singleton_cycle_builder::<TComponent>(),
            LifeCycle::ContextDependent => self.component_cycled_components_collection.blocking_write().add_context_dependent_cycle_builder::<TComponent>(),
        };
        //---------------------------

        Ok(ServiceMappingBuilder::new(self.clone()))
    }

    pub (crate) fn blocking_resolve<'a, TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<TService>{//std::pin::Pin<Box<dyn std::future::Future<Output = BuildDependencyResult<TService>> + Send + Sync + 'a>> {
        let service_id = TypeId::of::<TService>();

        let component_service_pair = self.cycled_component_service_collection.blocking_read().get_nth_by_service_type::<TService>(0)
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;
    
        let component_cycled_component_pair = self.component_cycled_components_collection.blocking_read()
            .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
            .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));
        
        #[cfg(feature = "loop-check")]
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            blocking_check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info)?;
        }
    
        let cycled_component = component_cycled_component_pair.converter.blocking_build(self.clone(), local_context)?;
        let service: Box<TService> = component_service_pair.converter.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

        Ok(Box::into_inner(service))
    }

    pub (crate) fn blocking_resolve_by_type_id<TService: Sync + Send + 'static>(self: &Arc<Self>, component_type_id: TypeId, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<TService> {
        let service_id = TypeId::of::<TService>();

        let component_service_pair = self.cycled_component_service_collection.blocking_read().get_all_by_service_type_with_cycled_component_id::<TService>(component_type_id)
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let component_cycled_component_pair = self.component_cycled_components_collection.blocking_read()
            .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
            .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));

        #[cfg(feature = "loop-check")]
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            blocking_check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info)?;
        }

        let cycled_component = component_cycled_component_pair.converter.blocking_build(self.clone(), local_context)?;
        let service: Box<TService> = component_service_pair.converter.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

        return Ok(Box::into_inner(service));
    }

    pub (crate) fn blocking_resolve_collection<TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<Vec<TService>> {
        let service_id = TypeId::of::<TService>();

        let component_service_pairs = self.cycled_component_service_collection.blocking_read().get_all_by_service_type::<TService>()
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let mut result = Vec::new();
        for component_service_pair in component_service_pairs.iter() {
            let component_cycled_component_pair = self.component_cycled_components_collection.blocking_read()
                .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
                .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));

            #[cfg(feature = "loop-check")]
            if let DependencyContextId::TypeId(type_info) = &id {
                // Link created on dependency add, we need take link for dependency, not cycled dependency or service
                blocking_check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info)?;
            }

            let cycled_component = component_cycled_component_pair.converter.blocking_build(self.clone(), local_context.clone())?;
            let service: Box<TService> = component_service_pair.converter.build(cycled_component)
                .downcast::<TService>()
                .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

            result.push(Box::into_inner(service));
        }

       return Ok(result);
    }

    pub fn blocking_map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(self: &Arc<Self>) -> MapComponentResult<Arc<Self>> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.components.blocking_read();
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{
                component_type_info: TypeInfo::from_type::<TComponent>(),
                service_type_info: TypeInfo::from_type::<TService>(),
            });
        }

        let component = component.unwrap();

        match component.life_cycle_type {
            LifeCycle::Transient => self.cycled_component_service_collection.blocking_write().add_mapping_as_transient::<TComponent, TService>(),
            LifeCycle::Singleton => self.cycled_component_service_collection.blocking_write().add_mapping_as_singleton::<TComponent, TService>(),
            LifeCycle::ContextDependent => self.cycled_component_service_collection.blocking_write().add_mapping_as_context_dependent::<TComponent, TService>(),
        };

        Ok(self.clone())
    }

    pub fn blocking_delete_component<TComponent: 'static>(&self) -> DeleteComponentResult<()> {
        // Get cycled_component_service_collection write guard at first, because resolve service start from cycled_component_service_collection read. Here we block resolve before delete
        // I dont make top RwLock, because in resolve time we can add new dependency from ctr, that mean write lock and then deadlock, because top level RwLock will be in read lock
        let mut cycled_component_service_collection_write_guard = self.cycled_component_service_collection.blocking_write();

        // We check life cycle & check component existence in one time
        let life_cycle = self.components.blocking_read().get(&TypeId::of::<TComponent>())
            .ok_or(DeleteComponentError::ComponentNotFound { component_type_info: TypeInfo::from_type::<TComponent>() })?.life_cycle_type;

        if ![LifeCycle::Singleton, LifeCycle::Transient].contains(&life_cycle) {
            return Err(DeleteComponentError::NotSupportedLifeCycle { component_type_info: TypeInfo::from_type::<TComponent>(), life_cycle })
        }

        let component = self.components.blocking_write()
            .remove(&TypeId::of::<TComponent>())
            .unwrap();

        #[cfg(feature = "loop-check")]
        let _ = self.links.blocking_write().remove(&TypeId::of::<TComponent>()).unwrap();

        let cycled_component_builder = self.component_cycled_components_collection.blocking_write()
            .delete_by_component::<TComponent>()
            .unwrap();

        match component.life_cycle_type {
            // Singleton mayby not exist, because not requested
            LifeCycle::Singleton => _ = self.global_context.blocking_write().singletons
                    .remove(&cycled_component_builder.cycled_component_type_info.type_id),
            LifeCycle::ContextDependent => panic!("Failed life cycle check before delete, life cycle:[{life_cycle:?}]"),
            _ => {}
        }

        _ = cycled_component_service_collection_write_guard.delete_cycled_component(&cycled_component_builder.cycled_component_type_info.type_id)
            .unwrap();
        
        Ok(())
    }

    #[inline(always)]
    pub (crate) fn blocking_is_service_exist(&self, service_type_id: TypeId) -> bool {
        self.cycled_component_service_collection.blocking_read().is_service_exist(&service_type_id)
    }

    #[inline(always)]
    pub (crate) fn blocking_is_component_exist(&self, component_type_id: TypeId) -> bool {
        self.components.blocking_read().contains_key(&component_type_id)
    }
}

#[cfg(not(feature = "async-mode"))]
impl CoreContext {
    pub (crate) fn register<TComponent: Sync + Send + 'static>(self: &Arc<Self>, ctor: Box<dyn ITypeConstructor>, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        let component = Component::new::<TComponent>(life_cycle.clone(), ctor);

        let component_id = component.component_type_info.type_id.clone();

        // Проверяем наличие зависимости, если нет добавляем
        let mut components_guard = self.components.write().unwrap();
        
        if components_guard.contains_key(&component.component_type_info.type_id) {
            return Err(AddDependencyError::DependencyExist { component_type_info: TypeInfo::from_type::<TComponent>() });
        }
   
        components_guard.insert(component_id.clone(), Arc::new(component));
        //---------------------------

        // Создаем ячейку свзяей без связей //TODO: в линках брать реальный ид
        #[cfg(feature = "loop-check")]
        self.links.write().unwrap().insert(component_id, DependencyLink::new());
        //---------------------------

        // Пустой маппинг сомпонента
        match life_cycle {
            LifeCycle::Transient => self.cycled_component_service_collection.write().unwrap().add_mapping_as_self::<TComponent>(),
            LifeCycle::Singleton => self.cycled_component_service_collection.write().unwrap().add_mapping_as_self::<Arc<TComponent>>(),
            LifeCycle::ContextDependent => self.cycled_component_service_collection.write().unwrap().add_mapping_as_self::<Weak<TComponent>>(),
        }
        //---------------------------

        // Регистрируем обработчик лайфтайма компонента
        match life_cycle {
            LifeCycle::Transient => self.component_cycled_components_collection.write().unwrap().add_transient_cycle_builder::<TComponent>(),
            LifeCycle::Singleton => self.component_cycled_components_collection.write().unwrap().add_singleton_cycle_builder::<TComponent>(),
            LifeCycle::ContextDependent => self.component_cycled_components_collection.write().unwrap().add_context_dependent_cycle_builder::<TComponent>(),
        };
        //---------------------------

        Ok(ServiceMappingBuilder::new(self.clone()))
    }

    pub (crate) fn resolve<'a, TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<TService>{//std::pin::Pin<Box<dyn std::future::Future<Output = BuildDependencyResult<TService>> + Send + Sync + 'a>> {
        let service_id = TypeId::of::<TService>();

        let component_service_pair = self.cycled_component_service_collection.read().unwrap().get_nth_by_service_type::<TService>(0)
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;
    
        let component_cycled_component_pair = self.component_cycled_components_collection.read().unwrap()
            .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
            .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));
        
        #[cfg(feature = "loop-check")]
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info)?;
        }
    
        let cycled_component = component_cycled_component_pair.converter.build(self.clone(), local_context)?;
        let service: Box<TService> = component_service_pair.converter.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

        Ok(Box::into_inner(service))
    }

    pub (crate) fn resolve_by_type_id<TService: Sync + Send + 'static>(self: &Arc<Self>, component_type_id: TypeId, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<TService> {
        let service_id = TypeId::of::<TService>();

        let component_service_pair = self.cycled_component_service_collection.read().unwrap().get_all_by_service_type_with_cycled_component_id::<TService>(component_type_id)
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let component_cycled_component_pair = self.component_cycled_components_collection.read().unwrap()
            .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
            .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));

        #[cfg(feature = "loop-check")]
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info)?;
        }

        let cycled_component = component_cycled_component_pair.converter.build(self.clone(), local_context)?;
        let service: Box<TService> = component_service_pair.converter.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

        return Ok(Box::into_inner(service));
    }

    pub (crate) fn resolve_collection<TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, local_context: Arc<LocalContext>) -> BuildDependencyResult<Vec<TService>> {
        let service_id = TypeId::of::<TService>();

        let component_service_pairs = self.cycled_component_service_collection.read().unwrap().get_all_by_service_type::<TService>()
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let mut result = Vec::new();
        for component_service_pair in component_service_pairs.iter() {
            let component_cycled_component_pair = self.component_cycled_components_collection.read().unwrap()
                .get_by_cycled_component_id(&component_service_pair.cycled_component_id)
                .expect(&format!("Component service pair exist but component cycled component pair not found:[{component_service_pair:?}]"));

            #[cfg(feature = "loop-check")]
            if let DependencyContextId::TypeId(type_info) = &id {
                // Link created on dependency add, we need take link for dependency, not cycled dependency or service
                check_link(self.clone(), &component_cycled_component_pair.component_type_info, type_info)?;
            }

            let cycled_component = component_cycled_component_pair.converter.build(self.clone(), local_context.clone())?;
            let service: Box<TService> = component_service_pair.converter.build(cycled_component)
                .downcast::<TService>()
                .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}], found [{component_cycled_component_pair:?}]", service_name = type_name::<TService>().to_string()));

            result.push(Box::into_inner(service));
        }

       return Ok(result);
    }

    pub fn map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(self: &Arc<Self>) -> MapComponentResult<Arc<Self>> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.components.read().unwrap();
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{
                component_type_info: TypeInfo::from_type::<TComponent>(),
                service_type_info: TypeInfo::from_type::<TService>(),
            });
        }

        let component = component.unwrap();

        match component.life_cycle_type {
            LifeCycle::Transient => self.cycled_component_service_collection.write().unwrap().add_mapping_as_transient::<TComponent, TService>(),
            LifeCycle::Singleton => self.cycled_component_service_collection.write().unwrap().add_mapping_as_singleton::<TComponent, TService>(),
            LifeCycle::ContextDependent => self.cycled_component_service_collection.write().unwrap().add_mapping_as_context_dependent::<TComponent, TService>(),
        };

        Ok(self.clone())
    }

    pub fn delete_component<TComponent: 'static>(&self) -> DeleteComponentResult<()> {
        // Get cycled_component_service_collection write guard at first, because resolve service start from cycled_component_service_collection read. Here we block resolve before delete
        // I dont make top RwLock, because in resolve time we can add new dependency from ctr, that mean write lock and then deadlock, because top level RwLock will be in read lock
        let mut cycled_component_service_collection_write_guard = self.cycled_component_service_collection.write().unwrap();

        // We check life cycle & check component existence in one time
        let life_cycle = self.components.read().unwrap().get(&TypeId::of::<TComponent>())
            .ok_or(DeleteComponentError::ComponentNotFound { component_type_info: TypeInfo::from_type::<TComponent>() })?.life_cycle_type;

        if ![LifeCycle::Singleton, LifeCycle::Transient].contains(&life_cycle) {
            return Err(DeleteComponentError::NotSupportedLifeCycle { component_type_info: TypeInfo::from_type::<TComponent>(), life_cycle })
        }

        let component = self.components.write().unwrap()
            .remove(&TypeId::of::<TComponent>())
            .unwrap();

        #[cfg(feature = "loop-check")]
        let _ = self.links.write().unwrap().remove(&TypeId::of::<TComponent>()).unwrap();

        let cycled_component_builder = self.component_cycled_components_collection.write().unwrap()
            .delete_by_component::<TComponent>()
            .unwrap();

        match component.life_cycle_type {
            // Singleton mayby not exist, because not requested
            LifeCycle::Singleton => _ = self.global_context.write().unwrap().singletons
                    .remove(&cycled_component_builder.cycled_component_type_info.type_id),
            LifeCycle::ContextDependent => panic!("Failed life cycle check before delete, life cycle:[{life_cycle:?}]"),
            _ => {}
        }

        _ = cycled_component_service_collection_write_guard.delete_cycled_component(&cycled_component_builder.cycled_component_type_info.type_id)
            .unwrap();
        
        Ok(())
    }

    #[inline(always)]
    pub (crate) fn is_service_exist(&self, service_type_id: TypeId) -> bool {
        self.cycled_component_service_collection.read().unwrap().is_service_exist(&service_type_id)
    }

    #[inline(always)]
    pub (crate) fn is_component_exist(&self, component_type_id: TypeId) -> bool {
        self.components.read().unwrap().contains_key(&component_type_id)
    }
}


#[cfg(all(feature = "loop-check", feature = "async-mode"))]
#[inline(always)]
async fn check_link(core_context: Arc<CoreContext>, child_type_info: &TypeInfo, parent_type_info: &TypeInfo) -> BuildDependencyResult<()> {
    let links_read_guard = core_context.links.read().await;

    let parent_links = links_read_guard.get(&parent_type_info.type_id)
        .expect(&format!("parent dependency link required TypeInfo:[{child_type_info:?}]"));

    // если связь уже проверена то все ок
    if parent_links.childs.contains(&child_type_info.type_id) {
        return Ok(());
    }

    // заранее (до write лока) валидируем зависимости, для возможности без write лока распознать ошибку
    if !validate_dependency(&links_read_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info.clone(),
            parent_type_info: parent_type_info.clone()
        })
    }

    drop(links_read_guard);
    // Необходима write блокировка, чтобы между зависимости в дереве не взяли write лок.
    // В этом случае может произойти взаимная блокировка, т. a <- @ <- b <- @ <- a <- b , между 'b' write лок зависимости 'a', между 'a' write лок зависимости 'b' 
    let mut links_write_guard = core_context.links.write().await;

    let parent_links = links_write_guard.get(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf? x2] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    // повторно валидируем зависимости, на случай, если во время разблокировки было изменено дерево связей
    // Получается оверхэд, т.к. 2 проверки, но этот оверхэд только для первого запроса, после валидация не будет происходить, т.к. связь будет сохранена
    if !validate_dependency(&links_write_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info.clone(),
            parent_type_info: parent_type_info.clone()
        })
    }

    // TODO: убрать вовторную выборку связей
    //Не придумал как повторно не доставать ссылку, и при этом не добавлять RwLock для каждой связи отдельно
    drop(parent_links);

    let parent_links = links_write_guard.get_mut(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf?] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    parent_links.childs.push(child_type_info.type_id);

    let child_links = links_write_guard.get_mut(&child_type_info.type_id)
        .expect(&format!("[we check is before, wtf?] child dependency link required TypeInfo:[{child_type_info:?}]"));

    child_links.parents.push(parent_type_info.type_id.clone());

    Ok(())
}

#[cfg(all(feature = "loop-check", feature = "blocking"))]
#[inline(always)]
fn blocking_check_link(core_context: Arc<CoreContext>, child_type_info: &TypeInfo, parent_type_info: &TypeInfo) -> BuildDependencyResult<()> {
    let links_read_guard = core_context.links.blocking_read();

    let parent_links = links_read_guard.get(&parent_type_info.type_id)
        .expect(&format!("parent dependency link required TypeInfo:[{child_type_info:?}]"));

    // если связь уже проверена то все ок
    if parent_links.childs.contains(&child_type_info.type_id) {
        return Ok(());
    }

    // заранее (до write лока) валидируем зависимости, для возможности без write лока распознать ошибку
    if !validate_dependency(&links_read_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info.clone(),
            parent_type_info: parent_type_info.clone()
        })
    }

    drop(links_read_guard);
    // Необходима write блокировка, чтобы между зависимости в дереве не взяли write лок.
    // В этом случае может произойти взаимная блокировка, т. a <- @ <- b <- @ <- a <- b , между 'b' write лок зависимости 'a', между 'a' write лок зависимости 'b' 
    let mut links_write_guard = core_context.links.blocking_write();

    let parent_links = links_write_guard.get(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf? x2] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    // повторно валидируем зависимости, на случай, если во время разблокировки было изменено дерево связей
    // Получается оверхэд, т.к. 2 проверки, но этот оверхэд только для первого запроса, после валидация не будет происходить, т.к. связь будет сохранена
    if !validate_dependency(&links_write_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info.clone(),
            parent_type_info: parent_type_info.clone()
        })
    }

    // TODO: убрать вовторную выборку связей
    //Не придумал как повторно не доставать ссылку, и при этом не добавлять RwLock для каждой связи отдельно
    drop(parent_links);

    let parent_links = links_write_guard.get_mut(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf?] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    parent_links.childs.push(child_type_info.type_id);

    let child_links = links_write_guard.get_mut(&child_type_info.type_id)
        .expect(&format!("[we check is before, wtf?] child dependency link required TypeInfo:[{child_type_info:?}]"));

    child_links.parents.push(parent_type_info.type_id.clone());

    Ok(())
}

#[cfg(all(feature = "loop-check", not(feature = "async-mode")))]
#[inline(always)]
fn check_link(core_context: Arc<CoreContext>, child_type_info: &TypeInfo, parent_type_info: &TypeInfo) -> BuildDependencyResult<()> {
    let links_read_guard = core_context.links.read().unwrap();

    let parent_links = links_read_guard.get(&parent_type_info.type_id)
        .expect(&format!("parent dependency link required TypeInfo:[{child_type_info:?}]"));

    // если связь уже проверена то все ок
    if parent_links.childs.contains(&child_type_info.type_id) {
        return Ok(());
    }

    // заранее (до write лока) валидируем зависимости, для возможности без write лока распознать ошибку
    if !validate_dependency(&links_read_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info.clone(),
            parent_type_info: parent_type_info.clone()
        })
    }

    drop(links_read_guard);
    // Необходима write блокировка, чтобы между зависимости в дереве не взяли write лок.
    // В этом случае может произойти взаимная блокировка, т. a <- @ <- b <- @ <- a <- b , между 'b' write лок зависимости 'a', между 'a' write лок зависимости 'b' 
    let mut links_write_guard = core_context.links.write().unwrap();

    let parent_links = links_write_guard.get(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf? x2] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    // повторно валидируем зависимости, на случай, если во время разблокировки было изменено дерево связей
    // Получается оверхэд, т.к. 2 проверки, но этот оверхэд только для первого запроса, после валидация не будет происходить, т.к. связь будет сохранена
    if !validate_dependency(&links_write_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info.clone(),
            parent_type_info: parent_type_info.clone()
        })
    }

    // TODO: убрать вовторную выборку связей
    //Не придумал как повторно не доставать ссылку, и при этом не добавлять RwLock для каждой связи отдельно
    drop(parent_links);

    let parent_links = links_write_guard.get_mut(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf?] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    parent_links.childs.push(child_type_info.type_id);

    let child_links = links_write_guard.get_mut(&child_type_info.type_id)
        .expect(&format!("[we check is before, wtf?] child dependency link required TypeInfo:[{child_type_info:?}]"));

    child_links.parents.push(parent_type_info.type_id.clone());

    Ok(())
}

#[cfg(feature = "loop-check")]
#[inline(always)]
fn validate_dependency<'a>(links_map: &HashMap<TypeId, DependencyLink>, parent_links: &DependencyLink, child_id: &TypeId) -> bool {
    let mut parents_collection = VecDeque::new();
    parents_collection.push_back(&parent_links.parents);
    
    while let Some(deep_parents_id) = parents_collection.pop_front() {
        if deep_parents_id.contains(child_id) {
            return false
        }

        for deep_parent_id in deep_parents_id.iter() {
            let deep_parent_parents = links_map.get(&deep_parent_id)
                .expect(&format!("deep parent link required TypeId:[{deep_parent_id:?}]"));

            parents_collection.push_back(&deep_parent_parents.parents);
        }
    }

    true
}