use core::fmt::Debug;
use std::{any::{Any, TypeId, type_name}, marker::{PhantomData, Unsize}, sync::{Arc, Weak}};
use derive_new::new;

use crate::types::TypeInfo;

pub (crate) trait IServiceConstructor where Self: Sync + Send + 'static {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send>;
    fn get_component_info(&self) -> TypeInfo;
    fn get_service_info(&self) -> TypeInfo;
    fn get_constructor_type(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<Self>(), type_name::<Self>().to_string())
    }
}

impl Debug for Box<dyn IServiceConstructor> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IServiceConstructor")
            .field("[META]:component_info", &self.get_component_info())
            .field("[META]:service_info", &self.get_service_info())
            .field("[META]:constructor_type", &self.get_constructor_type())
            .finish()
    }
}

#[derive(Debug, Default, new)]
pub (crate) struct BoxedTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
    #[new(default)] service_phantom_data: PhantomData<TService>,
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for BoxedTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component = component.downcast::<TComponent>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<TComponent>(), type_name = type_name::<TComponent>().to_string()));

        let service = component as Box<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }

    fn get_component_info(&self) -> TypeInfo { TypeInfo::from_type::<TComponent>() }
    fn get_service_info(&self) -> TypeInfo { TypeInfo::from_type::<Box<TService>>() }
}

#[derive(Debug, Default, new)]
pub (crate) struct ArcTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
    #[new(default)] service_phantom_data: PhantomData<TService>,
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for ArcTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component: Box<Arc<TComponent>> = component.downcast::<Arc<TComponent>>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<Arc<TComponent>>(), type_name = type_name::<Arc<TComponent>>().to_string()));

        let service = Box::into_inner(component) as Arc<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }

    fn get_component_info(&self) -> TypeInfo { TypeInfo::from_type::<Arc<TComponent>>() }
    fn get_service_info(&self) -> TypeInfo { TypeInfo::from_type::<Arc<TService>>() }
}

#[derive(Debug, Default, new)]
pub (crate) struct WeakTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
    #[new(default)] service_phantom_data: PhantomData<TService>,
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for WeakTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component: Box<Weak<TComponent>> = component.downcast::<Weak<TComponent>>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<TComponent>(), type_name = type_name::<TComponent>().to_string()));

        let service = Box::into_inner(component) as Weak<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }

    fn get_component_info(&self) -> TypeInfo { TypeInfo::from_type::<Weak<TComponent>>() }
    fn get_service_info(&self) -> TypeInfo { TypeInfo::from_type::<Weak<TService>>() }
}

#[derive(Debug, Default, new)]
pub (crate) struct NoLogicService<TComponent: 'static> {
    #[new(default)] component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: Sync + Send + 'static> IServiceConstructor for NoLogicService<TComponent> {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        return component;
    }

    fn get_component_info(&self) -> TypeInfo { TypeInfo::from_type::<TComponent>() }
    fn get_service_info(&self) -> TypeInfo { TypeInfo::from_type::<TComponent>() }
}