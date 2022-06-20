use std::marker::Unsize;
use std::{sync::Arc, marker::PhantomData, any::TypeId};

use crate::LifeCycle;
use crate::types::TypeInfo;
use crate::{core_context::CoreContext, types::{MapComponentError, MapComponentResult}};

pub struct ServiceMappingBuilder<TComponent: Sync + Send + 'static> {
    core_context: Arc<CoreContext>,
    pd: PhantomData<TComponent>
}

/// Map registered component to service
impl<TComponent: Sync + Send + 'static> ServiceMappingBuilder<TComponent> {
    pub (crate) fn new(core_context: Arc<CoreContext>) -> Self {
        Self {
            core_context,
            pd: PhantomData,
        }
    }

    /// Map component as service
    /// 
    ///# Example
    ///---
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_type::<SomeComponent>(LifeCycle::Transient).await
    ///     .unwrap()
    ///     .map_as::<dyn SomeService>().await
    ///     .unwrap();
    /// ```
    #[cfg(feature = "async-mode")]
    pub async fn map_as<TService: ?Sized + Sync + Send + 'static>(self) -> MapComponentResult<Self> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.core_context.components.read().await;
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{
                component_type_info: TypeInfo::from_type::<TComponent>(),
                service_type_info: TypeInfo::from_type::<TService>(),
            });
        }

        let component = component.unwrap();

        match component.life_cycle_type {
            LifeCycle::Transient => self.core_context.cycled_component_service_collection.write().await.add_mapping_as_transient::<TComponent, TService>(),
            LifeCycle::Singleton => self.core_context.cycled_component_service_collection.write().await.add_mapping_as_singleton::<TComponent, TService>(),
            LifeCycle::ContextDependent =>  self.core_context.cycled_component_service_collection.write().await.add_mapping_as_context_dependent::<TComponent, TService>()
        };

        drop(components_read_guard);

        Ok(self)
    }

    /// Map component as service (blocking version)
    /// 
    ///# Example
    ///---
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_type::<SomeComponent>(LifeCycle::Transient)
    ///     .unwrap()
    ///     .blocking_map_as::<dyn SomeService>()
    ///     .unwrap();
    /// ```
    #[cfg(feature = "blocking")]
    pub fn blocking_map_as<TService: ?Sized + Sync + Send + 'static>(self) -> MapComponentResult<Self> where TComponent: Unsize<TService> {
        std::thread::spawn(move || {
            let component_id = TypeId::of::<TComponent>();

            let components_read_guard = self.core_context.components.blocking_read();
            let component = components_read_guard.get(&component_id);

            if component.is_none() {
                return Err(MapComponentError::ComponentNotFound{
                    component_type_info: TypeInfo::from_type::<TComponent>(),
                    service_type_info: TypeInfo::from_type::<TService>(),
                });
            }

            let component = component.unwrap();

            match component.life_cycle_type {
                LifeCycle::Transient => self.core_context.cycled_component_service_collection.blocking_write().add_mapping_as_transient::<TComponent, TService>(),
                LifeCycle::Singleton => self.core_context.cycled_component_service_collection.blocking_write().add_mapping_as_singleton::<TComponent, TService>(),
                LifeCycle::ContextDependent =>  self.core_context.cycled_component_service_collection.blocking_write().add_mapping_as_context_dependent::<TComponent, TService>()
            };

            drop(components_read_guard);

            Ok(self)
        }).join().unwrap()
    }

    /// Map component as service
    /// 
    ///# Example
    ///---
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_type::<SomeComponent>(LifeCycle::Transient)
    ///     .unwrap()
    ///     .map_as::<dyn SomeService>()
    ///     .unwrap();
    /// ```
    #[cfg(not(feature = "async-mode"))]
    pub fn map_as<TService: ?Sized + Sync + Send + 'static>(self) -> MapComponentResult<Self> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.core_context.components.read().unwrap();
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{
                component_type_info: TypeInfo::from_type::<TComponent>(),
                service_type_info: TypeInfo::from_type::<TService>(),
            });
        }

        let component = component.unwrap();

        match component.life_cycle_type {
            LifeCycle::Transient => self.core_context.cycled_component_service_collection.write().unwrap().add_mapping_as_transient::<TComponent, TService>(),
            LifeCycle::Singleton => self.core_context.cycled_component_service_collection.write().unwrap().add_mapping_as_singleton::<TComponent, TService>(),
            LifeCycle::ContextDependent =>  self.core_context.cycled_component_service_collection.write().unwrap().add_mapping_as_context_dependent::<TComponent, TService>()
        };

        drop(components_read_guard);

        Ok(self)
    }
}