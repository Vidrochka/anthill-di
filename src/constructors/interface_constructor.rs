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
    async_ctor: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any>>>,
}

impl InterfaceConstructor {
    pub fn new<TInterface: ?Sized + 'static, TType: Constructor + Unsize<TInterface>>() -> Self {
        let ctor_wrapper: AsyncCallback<DependencyContext, BuildDependencyResult<Box<dyn Any>>> = Box::new(
            move |ctx: DependencyContext| -> _{
                Box::pin(
                    async move {
                        let instance = TType::ctor(ctx).await?;
                        let interface = Box::new(instance) as Box<TInterface>; 
                        Ok(Box::new(interface) as Box<dyn Any>)
                    }
                )
            }
        );
        Self { async_ctor: ctor_wrapper }
    }
}

#[async_trait(?Send)]
impl TypeConstructor for InterfaceConstructor {
    async fn ctor(&self, ctx: DependencyContext) -> BuildDependencyResult<Box<dyn Any>> {
        (self.async_ctor)(ctx).await
    }
}