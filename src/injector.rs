use std::{any::{TypeId, type_name}, collections::HashMap, sync::{Arc, Mutex}};

use crate::{builders::ContainerBuilder, container::Container};

pub struct Injector {
    containers: HashMap<TypeId, Container>
}

impl Injector {
    pub fn new(containers: Vec<Container>) -> Arc<Mutex<Self>> {
        let injector = Arc::new(Mutex::new(Self{containers: HashMap::new()}));

        let mut containers_map: HashMap<TypeId, Container> = HashMap::new();

        let mut containers = containers;

        containers.push(Container {
            type_id: TypeId::of::<Self>(),
            constructor: None,
            instance: Some(Box::new(Arc::clone(&injector))),
        });

        containers.push(ContainerBuilder::bind_type::<String>().build());
        containers.push(ContainerBuilder::bind_type::<&str>().build());
        containers.push(ContainerBuilder::bind_type::<u8>().build());
        containers.push(ContainerBuilder::bind_type::<i8>().build());
        containers.push(ContainerBuilder::bind_type::<u16>().build());
        containers.push(ContainerBuilder::bind_type::<i16>().build());
        containers.push(ContainerBuilder::bind_type::<u32>().build());
        containers.push(ContainerBuilder::bind_type::<i32>().build());
        containers.push(ContainerBuilder::bind_type::<u64>().build());
        containers.push(ContainerBuilder::bind_type::<i64>().build());
        containers.push(ContainerBuilder::bind_type::<u128>().build());
        containers.push(ContainerBuilder::bind_type::<i128>().build());
        containers.push(ContainerBuilder::bind_type::<usize>().build());
        containers.push(ContainerBuilder::bind_type::<isize>().build());
        containers.push(ContainerBuilder::bind_type::<f32>().build());
        containers.push(ContainerBuilder::bind_type::<f64>().build());
        containers.push(ContainerBuilder::bind_type::<bool>().build());

        containers.into_iter().rev().for_each(|container| 
        {
            containers_map.insert(container.type_id.clone(), container);
        });

        for (type_id, container) in &containers_map {
            println!("[injector::new] {:?} with constructor {:?}, with instance {:?}", 
                type_id, 
                container.constructor.is_some(),
                container.instance.is_some()
            )
        }

        injector.lock().unwrap().containers = containers_map;

        injector
    }

    pub fn get_singletone<TType>(&mut self) -> Result<Arc<Mutex<TType>>, crate::DiError> where TType: 'static {
        match self.containers.remove(&TypeId::of::<TType>()) {
            Some(mut container) => {
                let obj = container.build_singletone::<TType>(self);
                self.containers.insert(container.type_id.clone(), container);
                obj
                
            },
            None => Err(crate::DiError::ContainerNotFound{type_name: type_name::<TType>().to_string()}),
        }
    }

    pub fn get_new_instance<TType>(&mut self) -> Result<TType, crate::DiError> where TType: 'static {
        match self.containers.remove(&TypeId::of::<TType>()) {
            Some(container) => {
                let obj = container.build_new_instance::<TType>(self);
                self.containers.insert(container.type_id.clone(), container);
                obj
                
            },
            None => Err(crate::DiError::ContainerNotFound{type_name: type_name::<TType>().to_string()}),
        }
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.insert(container.type_id.clone(), container);
    }

    pub fn remove_container(&mut self, type_id: TypeId) -> Option<Container> {
        self.containers.remove(&type_id)
    }
}