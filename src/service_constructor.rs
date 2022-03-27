use core::fmt::Debug;
use std::{any::{Any, TypeId, type_name}, marker::{PhantomData, Unsize}, sync::Arc};

use crate::{core_context::DependencyCoreContext, types::TypeInfo};

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

pub (crate) struct BoxedTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    component_phantom_data: PhantomData<TComponent>,
    service_phantom_data: PhantomData<TService>,
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> BoxedTraitService<TComponent, TService> where TComponent: Unsize<TService> {
    #[must_use] pub (crate) fn new() -> Self { Self { component_phantom_data: PhantomData, service_phantom_data: PhantomData } }
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for BoxedTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component = component.downcast::<TComponent>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<TComponent>(), type_name = type_name::<TComponent>().to_string()));

        let service = component as Box<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }

    fn get_component_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }

    fn get_service_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<Box<TService>>(), type_name::<Box<TService>>().to_string())
    }
}

pub (crate) struct NoLogicService<TComponent: 'static> {
    component_phantom_data: PhantomData<TComponent>,
}

impl<TComponent: 'static> NoLogicService<TComponent> {
    #[must_use] pub (crate) fn new() -> Self { Self { component_phantom_data: PhantomData } }
}

impl<TComponent: Sync + Send + 'static> IServiceConstructor for NoLogicService<TComponent> {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        return component;
    }

    fn get_component_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }

    fn get_service_info(&self) -> TypeInfo {
        TypeInfo::new(TypeId::of::<TComponent>(), type_name::<TComponent>().to_string())
    }
}