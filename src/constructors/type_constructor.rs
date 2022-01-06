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
    async_ctor: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any>>>,
}

impl BaseConstructor {
    pub fn new<T>() -> Self where T: Constructor {
        let ctor_wrapper: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any>>> = Box::new(
            move |ctx: DependencyContext| -> _{
                Box::pin(
                    async move {
                        let instance = T::ctor(ctx).await?;
                        Ok(Box::new(instance) as Box<dyn Any>)
                    }
                )
            }
        );
        Self { async_ctor: ctor_wrapper }
    }
}

#[async_trait(?Send)]
impl TypeConstructor for BaseConstructor {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any>> {
        (self.async_ctor)(ctx).await
    }
}