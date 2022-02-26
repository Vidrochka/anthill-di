use std::{
    any::{
        TypeId,
        type_name
    },
    sync::{
        Arc,
        Weak
    }
};

use tokio::sync::RwLock;

use crate::{
    DependencyScope,
    DependencyCoreContext,
    Dependency,
    DependencyLink,
    DependencyContext,
    DependencyContextId,
    types::{
        BuildDependencyResult,
        BuildDependencyError
    },
    DependencyLifeCycle
};

pub struct DependencyBuilder {}

impl DependencyBuilder {
    pub (crate) async fn build<T: Sync + Send + 'static>(scope: Arc<DependencyScope>, ctx: Arc<DependencyCoreContext>) -> BuildDependencyResult<T> {
        let dependency = Self::get_dependency::<T>(&ctx).await;

        if let Some(dependency) = dependency {
            let dependency_context = DependencyContext::new_dependency(DependencyContextId::TypeId(TypeId::of::<T>()), ctx, scope);
            let new_instance_no_type = dependency.di_type.ctor.ctor(dependency_context).await?;

            match new_instance_no_type.downcast::<T>() {
                Ok(new_instance_with_type) => Ok(Box::into_inner(new_instance_with_type)),
                Err(_) => Err(BuildDependencyError::InvalidCast {
                    from_id: dependency.di_type.id.clone(),
                    from_name: dependency.di_type.name.clone(),
                    to_id: TypeId::of::<T>(),
                    to_name: type_name::<T>().to_string(),
                }),
            }
        } else {
            Err(BuildDependencyError::NotFound {
                id: TypeId::of::<T>(),
                name: type_name::<T>().to_string()
            })
        }
    }

    pub (crate) async fn try_add_link<TChild: 'static>(ctx: Arc<DependencyCoreContext>, parent_id: DependencyContextId) -> BuildDependencyResult<()> {
        let id = TypeId::of::<TChild>();

        let mut link_collection_guard = ctx.dependency_link_collection.write().await;

        if let DependencyContextId::TypeId(parent_id) = parent_id {
            // если есть родитель, то он ранее был создан и имеется ссвязь с ним
            // клон ссылки на связь чтобы отвязаться от лока всей коллекции
            let parent_link = link_collection_guard.get(&parent_id).unwrap().clone();
            
            if let Some(child_link) = link_collection_guard.get(&id) {
                // блокируем родительский объект на чтение для исключения параллельной проверки на зацикливание A->B и B->A
                let parent_link_guard = parent_link.read().await;

                // клон ссылки на связь чтобы отвязаться от лока всей коллекции
                let child_link = child_link.clone();
                // блокируем текущий объект для неизменности состояния связей
                let mut child_link_guard = child_link.write().await;

                drop(link_collection_guard); // заблокировали от изменений родительский и дочерний объект, нам больше не нужен лок всей коллекции

                // если есть такой родитель, то все уже проверено
                if child_link_guard.parents.contains_key(&parent_id) {
                    return Ok(());
                }
    
                // ищем ссылка на дочерний объект в родителях родительского объекта
                if parent_link_guard.search_link(&id).await {
                    return Err(BuildDependencyError::CyclicReference { id, name: type_name::<TChild>().to_string(), parent_id });
                }
        
                // лишнее клонирование, возможно можно измежать, но хз как т.к первоначальный объект залочен
                child_link_guard.add_parent(parent_id, parent_link.clone());
                return Ok(());
            }
    
            link_collection_guard.insert(id, Arc::new(RwLock::new(DependencyLink::with_parent(parent_id, parent_link))));
    
            return Ok(());
        }

        // если элемент корневой и нет в коллекции связей, добавляем
        if !link_collection_guard.contains_key(&id) {
            link_collection_guard.insert(id, Arc::new(RwLock::new(DependencyLink::new())));
        }
        
        return Ok(());
    }

    pub (crate) async fn get_dependency<T: 'static>(ctx: &Arc<DependencyCoreContext>) -> Option<Arc<Dependency>> {
        let id = TypeId::of::<T>();

        match ctx.dependency_collection.read().await.get(&id) {
            Some(d) => Some(d.clone()),
            None => None,
        }
    }
}