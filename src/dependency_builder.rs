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
    pub (crate) async fn build_transient<T: 'static>(scope: Arc<DependencyScope>, ctx: Arc<DependencyCoreContext>) -> BuildDependencyResult<T> {
        let dependency = DependencyBuilder::get_dependency::<T>(&ctx).await?;

        if dependency.life_cycle_type != DependencyLifeCycle::Transient {
            return Err(BuildDependencyError::InvalidLifeCycle {
                id: dependency.di_type.id.clone(),
                name: type_name::<T>().to_string(),
                expected: dependency.life_cycle_type.clone(),
                requested: DependencyLifeCycle::Transient,
            })
        }

        let dependency_context = DependencyContext::new_dependency(DependencyContextId::TypeId(TypeId::of::<T>()), ctx, scope);
        let new_instance_no_type = dependency.di_type.ctor.ctor(dependency_context).await?;

        match new_instance_no_type.downcast::<T>() {
            Ok(new_instance_with_type) => Ok(*new_instance_with_type),
            Err(_) => Err(BuildDependencyError::InvalidCast { id: dependency.di_type.id.clone(), name: type_name::<T>().to_string() }),
        }
    }

    pub (crate) async fn build_singleton<T: 'static>(scope: Arc<DependencyScope>, ctx: Arc<DependencyCoreContext>) -> BuildDependencyResult<Arc<RwLock<T>>> {
        let dependency = DependencyBuilder::get_dependency::<Arc<RwLock<T>>>(&ctx).await?;

        if dependency.life_cycle_type != DependencyLifeCycle::Singleton {
            return Err(BuildDependencyError::InvalidLifeCycle {
                id: dependency.di_type.id.clone(),
                name: type_name::<T>().to_string(),
                expected: dependency.life_cycle_type.clone(),
                requested: DependencyLifeCycle::Singleton,
            })
        }

        let mut singleton_dependency_guard = ctx.singleton_dependency.write().await;

        if let Some(singleton_instance_rw_lock) = singleton_dependency_guard.get_mut(&dependency.di_type.id) {
            let mut singleton_guard = singleton_instance_rw_lock.write().await;
            match (*singleton_guard).take() {
                Some(singleton_ref) => {
                    return match singleton_ref.downcast::<Arc<RwLock<T>>>() {
                        Ok(res) => {
                            let clone = (*res).clone();
                            singleton_guard.replace(res);
                            Ok(clone)
                        },
                        Err(_) => Err(BuildDependencyError::InvalidCast { id: dependency.di_type.id.clone(), name: type_name::<T>().to_string() }),
                    };
                },
                None => panic!("Singletone есть в коллекции, но не создан, ошибка логики"),
            }
        }

        let new_singleton = Arc::new(RwLock::new(None));
        singleton_dependency_guard.insert(dependency.di_type.id.clone(), new_singleton.clone());

        let mut add_singleton_guard = new_singleton.write().await;

        drop(singleton_dependency_guard);

        let dependency_context = DependencyContext::new_dependency(DependencyContextId::TypeId(TypeId::of::<Arc<RwLock<T>>>()), ctx.clone(), scope);
        let new_instance_no_type = dependency.di_type.ctor.ctor(dependency_context).await?;

        let new_instance = match new_instance_no_type.downcast::<T>() {
            Ok(new_instance_with_type) => *new_instance_with_type,
            Err(_) => return Err(BuildDependencyError::InvalidCast { id: dependency.di_type.id.clone(), name: type_name::<Arc<RwLock<T>>>().to_string() }),
        };

        let new_instance_ref = Arc::new(RwLock::new(new_instance));

        add_singleton_guard.replace(Box::new(new_instance_ref.clone()));

        Ok(new_instance_ref)
    }

    pub (crate) async fn build_scoped<T: 'static>(scope: Arc<DependencyScope>, ctx: Arc<DependencyCoreContext>) -> BuildDependencyResult<Weak<RwLock<T>>> {
        let dependency = DependencyBuilder::get_dependency::<Weak<RwLock<T>>>(&ctx).await?;

        if dependency.life_cycle_type != DependencyLifeCycle::Scoped {
            return Err(BuildDependencyError::InvalidLifeCycle {
                id: dependency.di_type.id.clone(),
                name: type_name::<T>().to_string(),
                expected: dependency.life_cycle_type.clone(),
                requested: DependencyLifeCycle::Scoped,
            })
        }
        
        let mut scope_dependency_guard = scope.scoped_dependencies.write().await;

        if let Some(scope_instance_rw_lock) = scope_dependency_guard.get_mut(&dependency.di_type.id) {
            let mut scoped_guard = scope_instance_rw_lock.write().await;
            match (*scoped_guard).take() {
                Some(scoped_ref) => {
                    //let scoped_clone = scoped_ref.clone();
                    return match scoped_ref.downcast::<Arc<RwLock<T>>>() {
                        Ok(res) => {
                            let clone = Arc::downgrade(&((*res).clone()));
                            scoped_guard.replace(res);
                            Ok(clone)
                        },
                        Err(_) => Err(BuildDependencyError::InvalidCast { id: dependency.di_type.id.clone(), name: type_name::<T>().to_string() }),
                    };
                },
                None => panic!("Singletone есть в коллекции, но не создан, ошибка логики"),
            }
        }

        let new_scoped = Arc::new(RwLock::new(None));
        scope_dependency_guard.insert(dependency.di_type.id.clone(), new_scoped.clone());

        let mut add_scoped_guard = new_scoped.write().await;

        drop(scope_dependency_guard);

        let dependency_context = DependencyContext::new_dependency(DependencyContextId::TypeId(TypeId::of::<Weak<RwLock<T>>>()), ctx, scope.clone());
        let new_instance_no_type = dependency.di_type.ctor.ctor(dependency_context).await?;

        let new_instance = match new_instance_no_type.downcast::<T>() {
            Ok(new_instance_with_type) => *new_instance_with_type,
            Err(_) => return Err(BuildDependencyError::InvalidCast { id: dependency.di_type.id.clone(), name: type_name::<Weak<RwLock<T>>>().to_string() }),
        };

        let new_instance_ref = Arc::new(RwLock::new(new_instance));

        add_scoped_guard.replace(Box::new(new_instance_ref.clone()));

        Ok(Arc::downgrade(&new_instance_ref))
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

    pub (crate) async fn get_dependency<T: 'static>(ctx: &Arc<DependencyCoreContext>) -> BuildDependencyResult<Arc<Dependency>> {
        let id = TypeId::of::<T>();

        match ctx.dependency_collection.read().await.get(&id) {
            Some(d) => Ok(d.clone()),
            None => Err(BuildDependencyError::NotFound { id, name: type_name::<T>().to_string() }),
        }
    }
}