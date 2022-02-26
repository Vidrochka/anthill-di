use std::{any::{TypeId, Any}, sync::{Arc, Weak}};
use tokio::sync::RwLock;
use crate::{TypeConstructor, DependencyContext, types::{BuildDependencyResult, AsyncCallback, AsyncBuilderCallback}, DependencyScope};


pub struct ScopedConstructor {
    arc_builder: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>>,
    arc_clone_closure: AsyncCallback<Arc<DependencyScope>, Option<Box<dyn Any + Sync + Send>>>,
}

fn get_arc_clone_closure<TType: Sync + Send + 'static>() -> AsyncCallback<Arc<DependencyScope>, Option<Box<dyn Any + Sync + Send>>> {
    Box::new(
        move |scope: Arc<DependencyScope>| -> _ {
            Box::pin (
                async move {
                    let scoped_dependencies_read_lock = scope.scoped_dependencies.read().await;
                    if let Some(scoped_instance) = scoped_dependencies_read_lock.get(&TypeId::of::<TType>()) {
                        let scoped_instance = scoped_instance.clone();
                        let scoped_instance_read_lock = scoped_instance.read().await;
                        let instance = match &*scoped_instance_read_lock {
                            Some(instance) => instance.clone(),
                            None => panic!("If instance exist in scope, expected write lock or filling state"),
                        };

                        let instance: Weak<TType> = Arc::downgrade(&instance.downcast::<TType>().unwrap());
                        return Some(Box::new(instance) as Box<dyn Any + Sync + Send>)
                    }

                    return None
                }
            )
        }
    )
}

impl ScopedConstructor {
    pub (crate) fn new<TType: Sync + Send + 'static>(type_ctor: Box<dyn TypeConstructor>) -> Self {
        let instance: Arc<RwLock<Option<Arc<dyn Any + Sync + Send>>>> = Arc::new(RwLock::new(None));
        let instance_clone = instance.clone(); 
        let type_ctor = Arc::new(type_ctor);

        let arc_builder: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>> = Box::new(
            move |ctx: DependencyContext| -> _{
                let instance_clone = instance_clone.clone();
                let type_ctor = type_ctor.clone();
                
                Box::pin (
                    async move {
                        let mut scoped_dependencies_write_lock = ctx.scope.scoped_dependencies.write().await;

                        // double check for check instance creating between lock
                        if let Some(scoped_instance) = scoped_dependencies_write_lock.get(&TypeId::of::<TType>()) {
                            let scoped_instance = scoped_instance.clone();
                            let scoped_instance_read_lock = scoped_instance.read().await;
                            let instance = match &*scoped_instance_read_lock {
                                Some(instance) => instance.clone(),
                                None => panic!("If instance exist in scope, expected write lock or filling state"),
                            };
                            let instance: Weak<TType> = Arc::downgrade(&instance.downcast::<TType>().unwrap());
                            return Ok(Box::new(instance) as Box<dyn Any + Sync + Send>)
                        }

                        let new_instance_ref = Arc::new(RwLock::new(None));
                        let new_instance_ref_clone = new_instance_ref.clone();

                        let mut new_instance_ref_write_guard = new_instance_ref.write().await;

                        scoped_dependencies_write_lock.insert(TypeId::of::<TType>(), new_instance_ref_clone);
                        drop(scoped_dependencies_write_lock);
                
                        let new_instance = type_ctor.ctor(ctx).await?;
                        let new_instance: Box<TType> = new_instance.downcast::<TType>().unwrap();
                        let new_instance = Arc::new(Box::into_inner(new_instance));
                        
                        *new_instance_ref_write_guard = Some(new_instance.clone() as Arc<dyn Any + Sync + Send>);

                        Ok(Box::new(Arc::downgrade(&new_instance)) as Box<dyn Any + Sync + Send>)
                    }
                )
            }
        );

        let arc_clone_closure = get_arc_clone_closure::<TType>();

        Self {
            arc_builder: arc_builder,
            arc_clone_closure: arc_clone_closure,
        }
    }
}

#[async_trait::async_trait]
impl TypeConstructor for ScopedConstructor {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let clone = (self.arc_clone_closure)(ctx.scope.clone()).await;

        if let Some(clone) = clone {
            return Ok(clone);
        }

        (self.arc_builder)(ctx).await
    }
}