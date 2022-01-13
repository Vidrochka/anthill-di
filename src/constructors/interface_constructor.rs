use std::{any::Any, marker::Unsize};

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

pub struct InterfaceConstructor {
    async_ctor: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>>,
}

impl InterfaceConstructor {
    pub fn new<TInterface: Sync + Send + ?Sized + 'static, TType: Constructor + Unsize<TInterface>>() -> Self {
        let ctor_wrapper: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any + Sync + Send>>> = Box::new(
            move |ctx: DependencyContext| -> _{
                Box::pin(
                    async move {
                        let instance = TType::ctor(ctx).await?;
                        let interface = Box::new(instance) as Box<TInterface>; 
                        Ok(Box::new(interface) as Box<dyn Any + Sync + Send>)
                    }
                )
            }
        );
        Self { async_ctor: ctor_wrapper }
    }
}

#[async_trait]
impl TypeConstructor for InterfaceConstructor {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any + Sync + Send>> {
        (self.async_ctor)(ctx).await
    }
}