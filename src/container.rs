use std::any::type_name_of_val;
use std::{any::{Any, TypeId, type_name}, sync::{Arc, Mutex}};

use crate::injector::Injector;

pub struct Container {
    pub type_id: TypeId,
    pub constructor: Option<Box<dyn Fn(&mut Injector) -> Box<dyn Any>>>,
    pub instance: Option<Box<dyn Any>>,
}

impl Container {
    pub fn build_singletone<TType>(&mut self, injector: &mut Injector) -> Arc<Mutex<TType>> where TType: 'static {
        if let Some(instance) = self.instance.take() {
            match instance.downcast::<Arc<Mutex<TType>>>() {
                Ok(typed_instance) =>
                {
                    let clone = Arc::clone(&*typed_instance);
                    self.instance = Some(typed_instance);
                    clone
                },
                Err(val) => {
                    let instanse_name = type_name_of_val(&*val);
                    self.instance = Some(val);
                    panic!("[container::build_singletone::select] invalid di cast type [{}] to [{}]", instanse_name, type_name::<Arc<Mutex<TType>>>())
                },
            }
        } else {
            if let Some(constructor) = &self.constructor {
                let constructed = (constructor)(injector);

                match constructed.downcast::<TType>() {
                    Ok(typed_constructed) => {
                        let constructed_singletone = Arc::new(Mutex::new(*typed_constructed));
                        self.instance = Some(Box::new(Arc::clone(&constructed_singletone)));
                        constructed_singletone
                    },
                    Err(val) => panic!("[container::build_singletone::create] invalid di cast type [{}] to [{}]", type_name_of_val(&*val), type_name::<TType>()),
                }
            } else {
                panic!("[container::build_singletone::create] type can not be constructed, constructor not defined [{}]", type_name::<TType>())
            }
        }
    }

    pub fn build_new_instance<TType>(&self, injector: &mut Injector) -> TType where TType: 'static {
        if let Some(constructor) = &self.constructor {
            let constructed = (constructor)(injector);

            match constructed.downcast::<TType>() {
                Ok(typed_constructed) => *typed_constructed,
                Err(val) => panic!("[build_new_instance] invalid di cast type [{}] to [{}]", type_name_of_val(&val), type_name::<TType>()),
            }
        } else {
            panic!("[container::build_new_instance] type can not be constructed, constructor not defined [{}]", type_name::<TType>())
        }
    }
}