use std::{any::{Any, TypeId}, marker::{PhantomData, Unsize}, sync::{Arc, Mutex}};

use crate::{container::Container, injection::Injection, injector::Injector};

pub struct InterfaceBuilder<TInterface, TType> 
where 
    TInterface: 'static + ?Sized,
    TType: Injection + Unsize<TInterface> + 'static
{
    pub phantom_interface: PhantomData<TInterface>,
    pub phantom_type: PhantomData<TType>,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Box<dyn Any>>>,
    pub instance: Option<Box<dyn Any>>,
}

impl<TInterface, TType> InterfaceBuilder<TInterface, TType> 
where 
    TInterface: 'static + ?Sized,
    TType: Injection + Unsize<TInterface> + 'static
{
    pub fn build(mut self) -> Container
    {
        if let None = self.constructor {
            let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(|injector: &mut Injector| -> Box<dyn Any> {
                let interface: Box<TInterface> = Box::new(TType::build_injection(injector)) as Box<TInterface>;
                Box::new(interface)
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
        self.instance = Some(Box::new(Arc::new(Mutex::new(value))));
        self
    }

    pub fn to_constructor(mut self, constructor: fn(&mut Injector) -> Box<TInterface>) -> Self {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(move |injector: &mut Injector| -> Box<dyn Any> {
            Box::new((constructor)(injector))
        });

        self.constructor = Some(constructor);
        self
    }
}