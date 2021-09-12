
use std::{any::{Any, TypeId}, marker::PhantomData, sync::{Arc, Mutex}};

use crate::{container::Container, injection::Injection, injector::Injector};

pub struct TypeBuilder<TType> where TType: Injection + 'static {
    pub phantom: PhantomData<TType>,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Box<dyn Any>>>,
    pub instance: Option<Box<dyn Any>>,
}

impl<TType> TypeBuilder<TType> where TType: Injection + 'static {
    pub fn build(mut self) -> Container{

        if let None = self.constructor {
            let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(|injector: &mut Injector| -> Box<dyn Any> {
                Box::new(TType::build_injection(injector))
            });
            self.constructor = Some(constructor)
        }

        Container {
            type_id: TypeId::of::<TType>(),
            constructor: self.constructor,
            instance: self.instance,
        }
    }

    pub fn to_value(mut self, value: TType) -> Self {
        self.instance = Some(Box::new(Arc::new(Mutex::new(value))));
        self
    }

    pub fn to_constructor(mut self, constructor: fn(&mut Injector) -> TType) -> Self {
        let constructor: Box<dyn Fn(&mut Injector) -> Box<dyn Any>> = Box::new(move |injector: &mut Injector| -> Box<dyn Any> {
            Box::new((constructor)(injector))
        });

        self.constructor = Some(constructor);
        self
    }
}