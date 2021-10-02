use std::any::type_name_of_val;
use std::{any::{Any, TypeId, type_name}, sync::Arc};

use crate::injector::Injector;

use tokio::sync::RwLock;

pub struct Container {
    pub type_id: TypeId,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Result<Box<dyn Any + Sync + Send>,crate::DiError> + Sync + Send>>,
    pub instance: Option<Arc<dyn Any + Sync + Send>>,
}

impl Container {
    pub fn build_singletone<TType>(&mut self, injector: &mut Injector) -> Result<Arc<RwLock<TType>>, crate::DiError>
    where
        TType: 'static + Sync + Send 
    {
        if let Some(instance) = &self.instance {
            match instance.clone().downcast::<Arc<RwLock<TType>>>() {
                Ok(typed_instance) =>
                {
                    let clone = Arc::clone(&*typed_instance);
                    //self.instance = Some(Arc::new(typed_instance));
                    Ok(clone)
                },
                Err(val) => {
                    let instanse_name = type_name_of_val(&*val);
                    //self.instance = Some(val);
                    Err(crate::DiError::IvalidDiCast{from: instanse_name.to_string(), to: type_name::<Arc<RwLock<TType>>>().to_string()})
                },
            }
        } else {
            if let Some(constructor) = &self.constructor {
                let constructed = (constructor)(injector);

                match constructed {
                    Ok(res) => {
                        match res.downcast::<TType>() {
                            Ok(typed_constructed) => {
                                let constructed_singletone = Arc::new(RwLock::new(*typed_constructed));
                                self.instance = Some(Arc::new(Arc::clone(&constructed_singletone)));
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

    pub fn build_new_instance<TType>(&self, injector: &mut Injector) -> Result<TType, crate::DiError>
    where
        TType: Sync + Send + 'static
    {
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