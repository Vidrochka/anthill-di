use std::{any::{Any, TypeId}, marker::{PhantomData, Unsize}, sync::Arc};

use crate::{container::Container, injection::Injection, injector::Injector};

use tokio::sync::RwLock;

pub struct InterfaceBuilder<TInterface, TType> 
where 
    TInterface: ?Sized + Sync + Send + 'static,
    TType: Injection + Unsize<TInterface> + Sync + Send + 'static
{
    pub phantom_interface: PhantomData<TInterface>,
    pub phantom_type: PhantomData<TType>,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send>>,
    pub instance: Option<Arc<dyn Any + Sync + Send>>,
}

impl<TInterface, TType> InterfaceBuilder<TInterface, TType> 
where 
    TInterface: ?Sized + Sync + Send + 'static,
    TType: Injection + Unsize<TInterface> + Sync + Send + 'static
{
    pub fn build(mut self) -> Container
    {
        if let None = self.constructor {
            let constructor: Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send> = Box::new(|injector: &mut Injector| -> Result<Box<dyn Any + Sync + Send>, crate::DiError> {
                let interface: Box<TInterface> = Box::new(TType::build_injection(injector)?) as Box<TInterface>;
                Ok(Box::new(interface))
            });
            self.constructor = Some(constructor)
        }

        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: self.constructor,
            instance: self.instance,
        }
    }

    pub fn to_value(mut self, value: Box<TInterface>) -> Self {
        self.instance = Some(Arc::new(Arc::new(RwLock::new(value))));
        self
    }

    pub fn to_constructor(mut self, constructor: fn(&mut Injector) -> Result<Box<TInterface>, crate::DiError>) -> Self {
        let constructor: Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>, crate::DiError> + Sync + Send> = Box::new(move |injector: &mut Injector| -> Result<Box<dyn Any + Sync + Send>, crate::DiError> {
            Ok(Box::new((constructor)(injector)?))
        });

        self.constructor = Some(constructor);
        self
    }
}