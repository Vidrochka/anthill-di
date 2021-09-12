use std::any::type_name_of_val;
use std::{any::{Any, TypeId, type_name}, sync::{Arc, Mutex}};

use crate::injector::Injector;

pub struct Container {
    pub type_id: TypeId,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any>,crate::DiError>>>,
    pub instance: Option<Box<dyn Any>>,
}

impl Container {
    pub fn build_singletone<TType>(&mut self, injector: &mut Injector) -> Result<Arc<Mutex<TType>>, crate::DiError> where TType: 'static {
        if let Some(instance) = self.instance.take() {
            match instance.downcast::<Arc<Mutex<TType>>>() {
                Ok(typed_instance) =>
                {
                    let clone = Arc::clone(&*typed_instance);
                    self.instance = Some(typed_instance);
                    Ok(clone)
                },
                Err(val) => {
                    let instanse_name = type_name_of_val(&*val);
                    self.instance = Some(val);
                    Err(crate::DiError::IvalidDiCast{from: instanse_name.to_string(), to: type_name::<Arc<Mutex<TType>>>().to_string()})
                },
            }
        } else {
            if let Some(constructor) = &self.constructor {
                let constructed = (constructor)(injector);

                match constructed {
                    Ok(res) => {
                        match res.downcast::<TType>() {
                            Ok(typed_constructed) => {
                                let constructed_singletone = Arc::new(Mutex::new(*typed_constructed));
                                self.instance = Some(Box::new(Arc::clone(&constructed_singletone)));
                                Ok(constructed_singletone)
                            },
                            Err(val) => Err(crate::DiError::IvalidDiCast{from: type_name_of_val(&*val).to_string(), to: type_name::<TType>().to_string()}),
                        }
                    },
                    Err(err) => Err(err)
                }
            } else {
                Err(crate::DiError::ConstructorNotDefined{type_name: type_name::<TType>().to_string()})
            }
        }
    }

    pub fn build_new_instance<TType>(&self, injector: &mut Injector) -> Result<TType, crate::DiError> where TType: 'static {
        if let Some(constructor) = &self.constructor {
            let constructed = (constructor)(injector);

            match constructed {
                Ok(res) => {
                    match res.downcast::<TType>() {
                        Ok(typed_constructed) => Ok(*typed_constructed),
                        Err(val) => Err(crate::DiError::IvalidDiCast{from: type_name_of_val(&*val).to_string(), to: type_name::<TType>().to_string()}),
                    }
                },
                Err(err) => Err(err)
            }
            
        } else {
            Err(crate::DiError::ConstructorNotDefined{type_name: type_name::<TType>().to_string()})
        }
    }
}