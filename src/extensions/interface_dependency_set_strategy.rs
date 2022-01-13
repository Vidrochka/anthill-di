use std::marker::Unsize;

use async_trait::async_trait;

use crate::{
    Constructor,
    DependencyContext,
    constructors::InterfaceConstructor,
    types::AddDependencyResult
};

#[async_trait(?Send)]
pub trait InterfaceDependencySetStrategy {
    async fn set_transient_interface<TInterface: ?Sized + 'static, TType: Constructor + Unsize<TInterface>>(&self) -> AddDependencyResult<()>;
    async fn set_singleton_interface<TInterface: Sync + Send + ?Sized + 'static, TType: Constructor + Unsize<TInterface>>(&self) -> AddDependencyResult<()>;
    async fn set_scoped_interface<TInterface: Sync + Send + ?Sized + 'static, TType: Constructor + Unsize<TInterface>>(&self) -> AddDependencyResult<()>;
}

#[async_trait(?Send)]
impl InterfaceDependencySetStrategy for DependencyContext {
    async fn set_transient_interface<TInterface: ?Sized + 'static, TType: Constructor + Unsize<TInterface>>(&self) -> AddDependencyResult<()> {
        self.add_transient::<Box<TInterface>>(Box::new(InterfaceConstructor::new::<TInterface, TType>())).await
    }

    async fn set_singleton_interface<TInterface: Sync + Send + ?Sized + 'static, TType: Constructor + Unsize<TInterface>>(&self) -> AddDependencyResult<()> {
        self.add_singleton::<Box<TInterface>>(Box::new(InterfaceConstructor::new::<TInterface, TType>())).await
    }

    async fn set_scoped_interface<TInterface: Sync + Send + ?Sized + 'static, TType: Constructor + Unsize<TInterface>>(&self) -> AddDependencyResult<()> {
        self.add_scoped::<Box<TInterface>>(Box::new(InterfaceConstructor::new::<TInterface, TType>())).await
    }
}