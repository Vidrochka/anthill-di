use std::{
    any::{
        TypeId,
        type_name
    },
    sync::{
        Arc,
    },
    collections::{HashMap, VecDeque}
};

use crate::{
    DependencyScope,
    DependencyCoreContext,
    DependencyLink,
    DependencyContext,
    DependencyContextId,
    types::{
        BuildDependencyResult,
        BuildDependencyError
    },
};

pub struct DependencyBuilder {}

impl DependencyBuilder {
    pub (crate) async fn build<T: Sync + Send + 'static>(scope: Arc<DependencyScope>, ctx: Arc<DependencyCoreContext>) -> BuildDependencyResult<T> {
        let dependency_id = TypeId::of::<T>();
        
        let dependency_context = DependencyContext::new_dependency(DependencyContextId::TypeId(TypeId::of::<T>(), type_name::<T>().to_string()), ctx.clone(), scope);

        let dependency = ctx.dependency_collection.read().await.get(&dependency_id)
            .expect(&format!("dependency not found, expected checked dependency TypeId:[{dependency_id:?}] type_name:[{type_name}]", type_name = type_name::<T>().to_string())).clone();
            
        let new_instance_no_type = dependency.di_type.ctor.ctor(dependency_context).await?;

        return match new_instance_no_type.downcast::<T>() {
            Ok(new_instance_with_type) => Ok(Box::into_inner(new_instance_with_type)),
            Err(_) => Err(BuildDependencyError::InvalidCast {
                from_id: dependency.di_type.id.clone(),
                from_name: dependency.di_type.name.clone(),
                to_id: dependency_id,
                to_name: type_name::<T>().to_string(),
            }),
        }
    }

    pub (crate) async fn try_add_link<TChild: 'static>(ctx: Arc<DependencyCoreContext>, parent_id: &TypeId, parent_name: &String) -> BuildDependencyResult<()> {
        let child_id = TypeId::of::<TChild>();

        let links_collection_read_guard = ctx.dependency_link_collection.read().await;

        let parent_links = links_collection_read_guard.get(&parent_id)
            .expect(&format!("parent dependency link required TypeId:[{parent_id:?}] type_name:[{parent_name:?}]"));

        // если связь уже проверена то все ок
        if parent_links.childs.contains(&parent_id) {
            return Ok(());
        }

        // заранее (до write лока) валидируем зависимости, для возможности без write лока распознать ошибку
        if !Self::validate_dependency(&links_collection_read_guard, parent_links, &child_id).await {
            return Err(BuildDependencyError::CyclicReference {
                child_id: child_id,
                child_name: type_name::<TChild>().to_string(),
                parent_id: parent_id.clone(),
                parent_name: parent_name.clone(),
            })
        }

        drop(links_collection_read_guard);
        // Необходима write блокировка, чтобы между зависимости в дереве не взяли write лок.
        // В этом случае может произойти взаимная блокировка, т. a <- @ <- b <- @ <- a <- b , между 'b' write лок зависимости 'a', между 'a' write лок зависимости 'b' 
        let mut links_collection_write_guard = ctx.dependency_link_collection.write().await;

        let parent_links = links_collection_write_guard.get(&parent_id)
            .expect(&format!("[we check is before, wtf? x2] parent dependency link required TypeId:[{parent_id:?}] type_name:[{parent_name:?}]"));

        // повторно валидируем зависимости, на случай, если во время разблокировки было изменено дерево связей
        // Получается оверхэд, т.к. 2 проверки, но этот оверхэд только для первого запроса, после валидация не будет происходить, т.к. связь будет сохранена
        if !Self::validate_dependency(&links_collection_write_guard, parent_links, &child_id).await {
            return Err(BuildDependencyError::CyclicReference {
                child_id: child_id,
                child_name: type_name::<TChild>().to_string(),
                parent_id: parent_id.clone(),
                parent_name: parent_name.clone(),
            })
        }

        // TODO: убрать вовторную выборку связей
        //Не придумал как повторно не доставать ссылку, и при этом не добавлять RwLock для каждой связи отдельно
        drop(parent_links);

        let parent_links = links_collection_write_guard.get_mut(&parent_id)
            .expect(&format!("[we check is before, wtf?] parent dependency link required TypeId:[{parent_id:?}] type_name:[{parent_name:?}]"));

        parent_links.childs.push(child_id);

        let child_links = links_collection_write_guard.get_mut(&child_id)
            .expect(&format!("[we check is before, wtf?] child dependency link required TypeId:[{child_id:?}] type_name:[{child_name:?}]", child_name = type_name::<TChild>().to_string()));

        child_links.parents.push(parent_id.clone());

        Ok(())
    }

    async fn validate_dependency<'a>(links_map: &HashMap<TypeId, DependencyLink>, parent_links: &DependencyLink, child_id: &TypeId) -> bool {
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
}