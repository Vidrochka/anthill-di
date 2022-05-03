use std::marker::Unsize;
use std::{sync::Arc, marker::PhantomData, any::TypeId};


use tokio::runtime::Builder;

use crate::DependencyLifeCycle;
use crate::types::TypeInfo;
use crate::{core_context::DependencyCoreContext, types::{MapComponentError, MapComponentResult}};

pub struct DependencyBuilder<TComponent: Sync + Send + 'static> {
    ctx: Arc<DependencyCoreContext>,
    pd: PhantomData<TComponent>
}

impl<TComponent: Sync + Send + 'static> DependencyBuilder<TComponent> {
    pub (crate) fn new(ctx: Arc<DependencyCoreContext>) -> Self {
        Self {
            ctx: ctx,
            pd: PhantomData,
        }
    }

    pub async fn map_as<TService: ?Sized + Sync + Send + 'static>(self) -> MapComponentResult<Self> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.ctx.components.read().await;
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{ type_info: TypeInfo::from_type::<TComponent>() });
        }

        let component = component.unwrap();

        let mut services_write_lock = self.ctx.services.write().await;

        match component.life_cycle_type {
            DependencyLifeCycle::Transient => services_write_lock.add_transient::<TComponent, TService>().await,
            DependencyLifeCycle::Singleton => services_write_lock.add_singleton::<TComponent, TService>().await,
            DependencyLifeCycle::Scoped => services_write_lock.add_scoped::<TComponent, TService>().await
        };

        drop(components_read_guard);
        drop(services_write_lock);

        Ok(self)
    }

    pub fn map_as_sync<TService: ?Sized + Sync + Send + 'static>(self) -> MapComponentResult<Self> where TComponent: Unsize<TService> {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async { self.map_as::<TService>().await })
    }
}