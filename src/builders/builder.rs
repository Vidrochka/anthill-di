use std::marker::{PhantomData, Unsize};

use crate::injection::Injection;

use super::{interface_builder::InterfaceBuilder, type_builder::TypeBuilder, unconfigured_interface_builder::UnconfiguredInterfaceBuilder, unconfigured_type_builder::UnconfiguredTypeBuilder};

pub struct ContainerBuilder {
}

impl ContainerBuilder {
    pub fn bind_interface<TInterface,TType>() -> InterfaceBuilder<TInterface,TType>
    where 
        TInterface: 'static + ?Sized,
        TType: Injection + Unsize<TInterface> + 'static
    {
        InterfaceBuilder{phantom_interface: PhantomData, phantom_type: PhantomData, constructor: None, instance: None}
    }

    pub fn bind_type<TType>() -> TypeBuilder<TType> where TType: Injection + 'static  {
        TypeBuilder{phantom: PhantomData, constructor: None, instance: None}
    }

    pub fn bind_unconfigured_type<TType>() -> UnconfiguredTypeBuilder<TType> where TType: 'static {
        UnconfiguredTypeBuilder{phantom: PhantomData}
    }

    pub fn bind_unconfigured_interface<TInterface, TType>() -> UnconfiguredInterfaceBuilder<TInterface, TType>
        where 
        TInterface: 'static + ?Sized,
        TType: Unsize<TInterface> + 'static
    {
        UnconfiguredInterfaceBuilder{phantom_interface: PhantomData, phantom_type: PhantomData}
    }
}