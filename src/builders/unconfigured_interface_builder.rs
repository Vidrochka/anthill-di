use std::{any::{Any, TypeId}, marker::{PhantomData, Unsize}, sync::Arc};

use crate::{Injector, Container};

use tokio::sync::RwLock;


pub struct UnconfiguredInterfaceBuilder<TInterface, TType>
where 
    TInterface: ?Sized + Send + Sync + 'static,
    TType: Unsize<TInterface> + Send + Sync + 'static
{
    pub phantom_interface: PhantomData<TInterface>,
    pub phantom_type: PhantomData<TType>,
}


impl<TInterface, TType> UnconfiguredInterfaceBuilder<TInterface, TType>
where 
    TInterface: ?Sized + Send + Sync + 'static,
    TType: Unsize<TInterface> + Send + Sync + 'static
{
    pub fn build_with_value(self, value: Box<TInterface>) -> Container {
        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: None,
            instance: Some(Arc::new(Arc::new(RwLock::new(value)))),
        }
    }

    pub fn build_with_constructor(self, constructor: fn(&mut Injector) -> Result<Box<TInterface>, crate::DiError>) -> Container {
        let constructor: Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send> = Box::new(move |injector: &mut Injector| -> Result<Box<dyn Any + Sync + Send>,crate::DiError> {
            Ok(Box::new((constructor)(injector)?))
        });

        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: Some(constructor),
            instance: None,
        }
    }

    pub fn build_with_constructor_and_value(self, value: Box<TInterface>, constructor: fn(&mut Injector) -> Result<Box<TInterface>, crate::DiError>) -> Container {
        let constructor: Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send> = Box::new(move |injector: &mut Injector| -> Result<Box<dyn Any + Sync + Send>,crate::DiError> {
            Ok(Box::new((constructor)(injector)?))
        });

        Container {
            type_id: TypeId::of::<Box<TInterface>>(),
            constructor: Some(constructor),
            instance: Some(Arc::new(Arc::new(RwLock::new(value)))),
        }
    }
}