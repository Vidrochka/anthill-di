use std::{any::{Any, TypeId}, marker::PhantomData, sync::{Arc, Mutex}};

use crate::{Injector, Container};


pub struct UnconfiguredTypeBuilder<TType> {
    pub phantom: PhantomData<TType>,
}


impl<TType> UnconfiguredTypeBuilder<TType> where TType: 'static {
    pub fn build_with_value(self, value: TType) -> Container {
        Container {
            type_id: TypeId::of::<TType>(),
            constructor: None,
            instance: Some(Box::new(Arc::new(Mutex::new(value)))),
        }
    }

    pub fn build_with_constructor(self, constructor: fn(&mut Injector) -> TType) -> Container {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(move |injector: &mut Injector| -> Box<dyn Any> {
            Box::new((constructor)(injector))
        });

        Container {
            type_id: TypeId::of::<TType>(),
            constructor: Some(constructor),
            instance: None,
        }
    }

    pub fn build_with_constructor_and_value(self, value: TType, constructor: fn(&mut Injector) -> TType) -> Container {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(move |injector: &mut Injector| -> Box<dyn Any> {
            Box::new((constructor)(injector))
        });

        Container {
            type_id: TypeId::of::<TType>(),
            constructor: Some(constructor),
            instance: Some(Box::new(Arc::new(Mutex::new(value)))),
        }
    }
}