use std::{
    collections::{HashMap, VecDeque},
    any::{TypeId, type_name},
    sync::{Arc, Weak},
    fmt::Debug, marker::Unsize
};

use tokio::sync::RwLock;
use derive_new::new;

use crate::{
    Dependency,
    DependencyLink,
    service::ServicesMappingsCollection,
    ICycledComponentBuilder,
    GlobalScope, DependencyType, DependencyLifeCycle, types::{AddDependencyResult, TypeInfo, AddDependencyError, BuildDependencyResult, BuildDependencyError, MapComponentError, MapComponentResult}, DependencyBuilder, cycled_component_builder::{TransientComponentBuilder, SingletonComponentBuilder, ScopedComponentBuilder}, DependencyContextId, DependencyScope
};

//#[derive(Debug)]
#[derive(Default, new)]
pub struct DependencyCoreContext where Self: Sync + Send {
    #[new(default)] pub (crate) components: RwLock<HashMap<TypeId, Arc<Dependency>>>,
    #[new(default)] pub (crate) cycled_component_builders: RwLock<HashMap<TypeId, Arc<Box<dyn ICycledComponentBuilder>>>>,
    #[new(default)] pub (crate) services: RwLock<ServicesMappingsCollection>,
    #[new(default)] pub (crate) links: RwLock<HashMap<TypeId, DependencyLink>>,
    #[new(default)] pub (crate) global_scope: Arc<RwLock<GlobalScope>>,
}

impl Debug for DependencyCoreContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependencyCoreContext")
            .field("cycled_component_builders", &self.cycled_component_builders.try_read().unwrap())
            .field("components", &self.components.try_read().unwrap())
            .field("services", &self.services.try_read().unwrap())
            .field("links", &self.links.try_read().unwrap())
            .field("global_scope", &self.global_scope.try_read().unwrap())
            .finish()
    }
}

impl DependencyCoreContext {
    pub (crate) async fn register<TComponent: Sync + Send + 'static>(self: &Arc<Self>, component_type: DependencyType, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
    //pub (crate) fn register<TComponent: Sync + Send + 'static>(self: &Arc<Self>, component_type: DependencyType, life_cycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TComponent>> {
        let component = Dependency::new(life_cycle.clone(), component_type);

        let component_id = component.di_type.id.clone();

        // Проверяем наличие зависимости, если нет добавляем
        let mut components_guard = self.components.write().await;
        
        if components_guard.contains_key(&component.di_type.id) {
            return Err(AddDependencyError::DependencyExist { type_info: TypeInfo::from_type::<TComponent>() });
        }
   
        components_guard.insert(component_id.clone(), Arc::new(component));
        //---------------------------

        // Создаем ячейку свзяей без связей //TODO: в линках брать реальный ид
        let mut links_guard = self.links.write().await;
        links_guard.insert(component_id.clone(), DependencyLink::new());
        //---------------------------

        // Пустой маппинг сомпонента
        match life_cycle {
            DependencyLifeCycle::Transient => self.services.write().await.add_no_mappings::<TComponent>().await,
            DependencyLifeCycle::Singleton => self.services.write().await.add_no_mappings::<Arc<TComponent>>().await,
            DependencyLifeCycle::Scoped => self.services.write().await.add_no_mappings::<Weak<TComponent>>().await,
        }
        //---------------------------

        // Регистрируем обработчик лайфтайма компонента
        let mut cycle_builders_write_guard = self.cycled_component_builders.write().await;

        let cycled_component_builder: Box<dyn ICycledComponentBuilder> = match life_cycle {
            DependencyLifeCycle::Transient => Box::new(TransientComponentBuilder::<TComponent>::new()),
            DependencyLifeCycle::Singleton => Box::new(SingletonComponentBuilder::<TComponent>::new()),
            DependencyLifeCycle::Scoped => Box::new(ScopedComponentBuilder::<TComponent>::new()),
        };

        cycle_builders_write_guard.insert(cycled_component_builder.get_output_type_info().type_id, Arc::new(cycled_component_builder));

        //---------------------------

        Ok(DependencyBuilder::new(self.clone()))
    }

    /// Resolve first (by TypeId) dependency
    pub (crate) async fn resolve<'a, TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, scope: Arc<DependencyScope>) -> BuildDependencyResult<TService>{//std::pin::Pin<Box<dyn std::future::Future<Output = BuildDependencyResult<TService>> + Send + Sync + 'a>> {
        let service_id = TypeId::of::<TService>();

        let services = self.services.read().await.get_all_collection_by_service_type::<TService>()
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;
    
        let services_read_lock = services.read().await;
    
        // { cycled_component_type_id , service_constructor }
        let service_info = services_read_lock.get_nth_service_info(0);
    
        let cycled_component_builder = self.cycled_component_builders.read().await
            .get(&service_info.0)
            .expect(&format!("Service exist but cycled component builder not found service_id:[{service_id:?}]", service_id = service_info.0))
            .clone();
    
        let component_info = cycled_component_builder.get_input_type_info();
    
        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            check_link(self.clone(), component_info, type_info).await?;
        }
    
        let cycled_component = cycled_component_builder.build(self.clone(), scope).await?;
        let service: Box<TService> = service_info.1.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}]", service_name = type_name::<TService>().to_string()));
    
        Ok(Box::into_inner(service))
    }

    pub (crate) async fn resolve_by_type_id<TService: Sync + Send + 'static>(self: &Arc<Self>, component_type_id: TypeId, id: DependencyContextId, scope: Arc<DependencyScope>) -> BuildDependencyResult<TService> {
        let service_id = TypeId::of::<TService>();

        let services = self.services.read().await.get_all_collection_by_service_type::<TService>()
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let services_read_lock = services.read().await;

        // { cycled_component_type_id , service_constructor }
        let service_constructor = services_read_lock.get_by_type_id(&component_type_id);

        let cycled_component_builder = self.cycled_component_builders.read().await
            .get(&component_type_id)
            .expect(&format!("Service exist but cycled component builder not found service_id:[{component_type_id:?}]"))
            .clone();

        let component_info = cycled_component_builder.get_input_type_info();

        if let DependencyContextId::TypeId(type_info) = &id {
            // Link created on dependency add, we need take link for dependency, not cycled dependency or service
            check_link(self.clone(), component_info, type_info).await?;
        }

        let cycled_component = cycled_component_builder.build(self.clone(), scope).await?;
        let service: Box<TService> = service_constructor.build(cycled_component)
            .downcast::<TService>()
            .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}]", service_name = type_name::<TService>().to_string()));

        return Ok(Box::into_inner(service));
    }

    pub (crate) async fn resolve_collection<TService: Sync + Send + 'static>(self: &Arc<Self>, id: DependencyContextId, scope: Arc<DependencyScope>) -> BuildDependencyResult<Vec<TService>> {
        let service_id = TypeId::of::<TService>();

        let services = self.services.read().await.get_all_collection_by_service_type::<TService>()
            .ok_or(BuildDependencyError::NotFound{ type_info: TypeInfo::from_type::<TService>() })?;

        let services_read_lock = services.read().await;

        // { cycled_component_type_id , service_constructor }
        let services_info = services_read_lock.get_all_services_info();

        let mut result = Vec::new();
        for service_info in services_info.iter() {
            let cycled_component_builder = self.cycled_component_builders.read().await
            .get(&service_info.0)
            .expect(&format!("Service exist but cycled component builder not found service_id:[{service_id:?}]", service_id = service_info.0))
            .clone();

            let component_info = cycled_component_builder.get_input_type_info();

            if let DependencyContextId::TypeId(type_info) = &id {
                // Link created on dependency add, we need take link for dependency, not cycled dependency or service
                check_link(self.clone(), component_info, type_info).await?;
            }

            let cycled_component = cycled_component_builder.build(self.clone(), scope.clone()).await?;
            let service: Box<TService> = service_info.1.build(cycled_component)
                .downcast::<TService>()
                .expect(&format!("Invalid service cast expected service_id:[{service_id:?}] service_name:[{service_name}]", service_name = type_name::<TService>().to_string()));

            result.push(Box::into_inner(service));
        }

        
       return Ok(result);
    }

    pub async fn map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(self: &Arc<Self>) -> MapComponentResult<Arc<Self>> where TComponent: Unsize<TService> {
        let component_id = TypeId::of::<TComponent>();

        let components_read_guard = self.components.read().await;
        let component = components_read_guard.get(&component_id);

        if component.is_none() {
            return Err(MapComponentError::ComponentNotFound{ type_info: TypeInfo::from_type::<TComponent>() });
        }

        let component = component.unwrap();

        let mut services_write_lock = self.services.write().await;

        match component.life_cycle_type {
            DependencyLifeCycle::Transient => services_write_lock.add_transient::<TComponent, TService>().await,
            DependencyLifeCycle::Singleton => services_write_lock.add_singleton::<TComponent, TService>().await,
            DependencyLifeCycle::Scoped => services_write_lock.add_scoped::<TComponent, TService>().await
        };

        Ok(self.clone())
    }
}

async fn check_link(ctx: Arc<DependencyCoreContext>, child_type_info: TypeInfo, parent_type_info: &TypeInfo) -> BuildDependencyResult<()> {
    let links_read_guard = ctx.links.read().await;

    let parent_links = links_read_guard.get(&parent_type_info.type_id)
        .expect(&format!("parent dependency link required TypeInfo:[{child_type_info:?}]"));

    // если связь уже проверена то все ок
    if parent_links.childs.contains(&child_type_info.type_id) {
        return Ok(());
    }

    // заранее (до write лока) валидируем зависимости, для возможности без write лока распознать ошибку
    if !validate_dependency(&links_read_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info,
            parent_type_info: parent_type_info.clone()
        })
    }

    drop(links_read_guard);
    // Необходима write блокировка, чтобы между зависимости в дереве не взяли write лок.
    // В этом случае может произойти взаимная блокировка, т. a <- @ <- b <- @ <- a <- b , между 'b' write лок зависимости 'a', между 'a' write лок зависимости 'b' 
    let mut links_write_guard = ctx.links.write().await;

    let parent_links = links_write_guard.get(&parent_type_info.type_id)
        .expect(&format!("[we check is before, wtf? x2] parent dependency link required TypeInfo:[{parent_type_info:?}]"));

    // повторно валидируем зависимости, на случай, если во время разблокировки было изменено дерево связей
    // Получается оверхэд, т.к. 2 проверки, но этот оверхэд только для первого запроса, после валидация не будет происходить, т.к. связь будет сохранена
    if !validate_dependency(&links_write_guard, parent_links, &child_type_info.type_id) {
        return Err(BuildDependencyError::CyclicReference {
            child_type_info: child_type_info,
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