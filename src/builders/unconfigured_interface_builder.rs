use std::{any::{Any, TypeId}, marker::{PhantomData, Unsize}, sync::{Arc, Mutex}};

use crate::{Injector, Container};


pub struct UnconfiguredInterfaceBuilder<TInterface, TType>
where 
    TInterface: 'static + ?Sized,
    TType: Unsize<TInterface> + 'static
{
    pub phantom_interface: PhantomData<TInterface>,
    pub phantom_type: PhantomData<TType>,
}


impl<TInterface, TType> UnconfiguredInterfaceBuilder<TInterface, TType>
where 
    TInterface: 'static + ?Sized,
    TType: Unsize<TInterface> + 'static
{
    pub fn build_with_value(self, value: Box<TInterface>) -> Container {
        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: None,
            instance: Some(Box::new(Arc::new(Mutex::new(value)))),
        }
    }

    pub fn build_with_constructor(self, constructor: fn(&mut Injector) -> Box<TInterface>) -> Container {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(move |injector: &mut Injector| -> Box<dyn Any> {
            Box::new((constructor)(injector))
        });

        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: Some(constructor),
            instance: None,
        }
    }

    pub fn build_with_constructor_and_value(self, value: Box<TInterface>, constructor: fn(&mut Injector) -> Box<TInterface>) -> Container {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(move |injector: &mut Injector| -> Box<dyn Any> {
            Box::new((constructor)(injector))
        });

        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: Some(constructor),
            instance: Some(Box::new(Arc::new(Mutex::new(value)))),
        }
    }
}