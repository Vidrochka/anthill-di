use tokio::sync::RwLock;
use std::{any::Any, sync::Arc};
use crate::{TypeConstructor, DependencyContext, types::{BuildDependencyResult, AsyncCallback, AsyncBuilderCallback}};


pub struct SingletonConstructor {
    arc_builder: Option<AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>>>,
    arc_clone_closure: AsyncBuilderCallback<Option<Box<dyn Any + Sync + Send>>>,
}

fn get_arc_clone_closure<TType: Sync + Send + 'static>(instance: Arc<RwLock<Option<Arc<dyn Any + Sync + Send>>>>) -> AsyncBuilderCallback<Option<Box<dyn Any + Sync + Send>>> {
    Box::new(
        move || -> _{
            let instance = instance.clone();
            Box::pin (
                async move {
                    let instance_read_guard = instance.read().await;

                    if let Some(instance) = &*instance_read_guard {
                        let instance: Arc<TType> = instance.clone().downcast::<TType>().unwrap();
                        return Some(Box::new(instance) as Box<dyn Any + Sync + Send>)
                    }

                    return None
                }
            )
        }
    )
}

impl SingletonConstructor {
    pub (crate) fn new_with_instance<TType: Sync + Send + 'static>(instance: Arc<TType>) -> Self {
        let instance: Arc<RwLock<Option<Arc<dyn Any + Sync + Send>>>> = Arc::new(RwLock::new(Some(instance)));
        let arc_clone_closure = get_arc_clone_closure::<TType>(instance);

        Self {
            arc_builder: None,
            arc_clone_closure: arc_clone_closure,
        }
    }

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
                        let mut instance_write_guard = instance_clone.write().await;
                
                        // double check for check instance creating between lock
                        if let Some(instance) = &*instance_write_guard {
                            let instance: Arc<TType> = instance.clone().downcast::<TType>().unwrap();
                            return Ok(Box::new(instance) as Box<dyn Any + Sync + Send>)
                        }
                
                        let new_instance = type_ctor.ctor(ctx).await?;
                        let new_instance: Box<TType> = new_instance.downcast::<TType>().unwrap();
                        let new_instance = Arc::new(Box::into_inner(new_instance));
                        
                        *instance_write_guard = Some(new_instance.clone() as Arc<dyn Any + Sync + Send>);

                        Ok(Box::new(new_instance) as Box<dyn Any + Sync + Send>)
                    }
                )
            }
        );

        let arc_clone_closure = get_arc_clone_closure::<TType>(instance);

        Self {
            arc_builder: Some(arc_builder),
            arc_clone_closure: arc_clone_closure,
        }
    }
}

#[async_trait::async_trait]
impl TypeConstructor for SingletonConstructor {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        let clone = (self.arc_clone_closure)().await;

        if let Some(clone) = clone {
            return Ok(clone);
        }

        match &self.arc_builder {
            Some(builder) => (builder)(ctx).await,
            _ => panic!("Singleton builder and instance not set"),
        }
    }
}