use std::any::Any;

use async_trait::async_trait;

use crate::{
    DependencyContext,
    types::{
        BuildDependencyResult,
        AsyncCallback
    },
    TypeConstructor,
    Constructor
};

pub struct BaseConstructor {
    async_ctor: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>>,
}

impl BaseConstructor {
    pub fn new<T>() -> Self where T: Constructor + Sync + Send {
        let ctor_wrapper: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>> = Box::new(
            move |ctx: DependencyContext| -> _{
                Box::pin(
                    async move {
                        let instance = T::ctor(ctx).await?;
                        Ok(Box::new(instance) as Box<dyn Any + Sync + Send>)
                    }
                )
            }
        );
        Self { async_ctor: ctor_wrapper }
    }
}

#[async_trait]
impl TypeConstructor for BaseConstructor {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        (self.async_ctor)(ctx).await
    }
}