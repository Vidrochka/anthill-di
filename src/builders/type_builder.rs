
use std::{any::{Any, TypeId}, marker::PhantomData, sync::Arc};

use crate::{container::Container, injection::Injection, injector::Injector};

use tokio::sync::RwLock;

pub struct TypeBuilder<TType> where TType: Injection + Send + Sync + 'static {
    pub phantom: PhantomData<TType>,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send>>,
    pub instance: Option<Arc<dyn Any + Send + Sync>>,
}

impl<TType> TypeBuilder<TType> where TType: Injection + Send + Sync + 'static {
    pub fn build(mut self) -> Container{

        if let None = self.constructor {
            let constructor: Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send> = Box::new(|injector: &mut Injector| -> Result<Box<dyn Any + Sync + Send>,crate::DiError> {
                Ok(Box::new(TType::build_injection(injector)?))
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
        self.instance = Some(Arc::new(Arc::new(RwLock::new(value))));
        self
    }

    pub fn to_constructor(mut self, constructor: fn(&mut Injector) -> Result<TType, crate::DiError>) -> Self {
        let constructor: Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send> = Box::new(move |injector: &mut Injector| -> Result<Box<dyn Any + Sync + Send>,crate::DiError> {
            Ok(Box::new((constructor)(injector)?))
        });

        self.constructor = Some(constructor);
        self
    }
}