use core::fmt::Debug;
use std::{any::{Any, TypeId, type_name}, marker::{PhantomData, Unsize}, sync::{Arc, Weak}};

pub (crate) trait IServiceConstructor where Self: Debug + Sync + Send + 'static {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send>;
}

pub (crate) struct BoxedTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    component_phantom_data: PhantomData<TComponent>,
    service_phantom_data: PhantomData<TService>,

    #[cfg(feature = "debug-type-info")]
    debug_component_type_info: TypeInfo,

    #[cfg(feature = "debug-type-info")]
    debug_service_type_info: TypeInfo,
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> Debug for BoxedTraitService<TComponent, TService>
where TComponent: Unsize<TService>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("BoxedTraitService");
        debug_struct.field("component_phantom_data", &self.component_phantom_data)
            .field("service_phantom_data", &self.service_phantom_data);

        #[cfg(feature = "debug-type-info")]
        debug_struct.field("debug_component_type_info", &self.debug_component_type_info)
            .field("debug_service_type_info", &self.debug_service_type_info);

        debug_struct.finish()
    }
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> BoxedTraitService<TComponent, TService>
where TComponent: Unsize<TService>
{
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
            service_phantom_data: Default::default(),
            #[cfg(feature = "debug-type-info")]
            debug_component_type_info: TypeInfo::from_type::<TComponent>(),
            #[cfg(feature = "debug-type-info")]
            debug_service_type_info: TypeInfo::from_type::<Box<TService>>(),
        }
    }
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for BoxedTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component = component.downcast::<TComponent>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<TComponent>(), type_name = type_name::<TComponent>().to_string()));

        let service = component as Box<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }
}

pub (crate) struct ArcTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    component_phantom_data: PhantomData<TComponent>,
    service_phantom_data: PhantomData<TService>,

    #[cfg(feature = "debug-type-info")]
    debug_component_type_info: TypeInfo,

    #[cfg(feature = "debug-type-info")]
    debug_service_type_info: TypeInfo,
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> Debug for ArcTraitService<TComponent, TService>
where TComponent: Unsize<TService>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("ArcTraitService");
        debug_struct.field("component_phantom_data", &self.component_phantom_data)
            .field("service_phantom_data", &self.service_phantom_data);

        #[cfg(feature = "debug-type-info")]
        debug_struct.field("debug_component_type_info", &self.debug_component_type_info)
            .field("debug_service_type_info", &self.debug_service_type_info);

        debug_struct.finish()
    }
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> ArcTraitService<TComponent, TService>
where TComponent: Unsize<TService>
{
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
            service_phantom_data: Default::default(),
            #[cfg(feature = "debug-type-info")]
            debug_component_type_info: TypeInfo::from_type::<Arc<TComponent>>(),
            #[cfg(feature = "debug-type-info")]
            debug_service_type_info: TypeInfo::from_type::<Arc<TService>>(),
        }
    }
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for ArcTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component: Box<Arc<TComponent>> = component.downcast::<Arc<TComponent>>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<Arc<TComponent>>(), type_name = type_name::<Arc<TComponent>>().to_string()));

        let service = Box::into_inner(component) as Arc<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }
}

pub (crate) struct WeakTraitService<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> where TComponent: Unsize<TService> {
    component_phantom_data: PhantomData<TComponent>,
    service_phantom_data: PhantomData<TService>,

    #[cfg(feature = "debug-type-info")]
    debug_component_type_info: TypeInfo,

    #[cfg(feature = "debug-type-info")]
    debug_service_type_info: TypeInfo,
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> Debug for WeakTraitService<TComponent, TService>
where TComponent: Unsize<TService>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("WeakTraitService");
        debug_struct.field("component_phantom_data", &self.component_phantom_data)
            .field("service_phantom_data", &self.service_phantom_data);

        #[cfg(feature = "debug-type-info")]
        debug_struct.field("debug_component_type_info", &self.debug_component_type_info)
            .field("debug_service_type_info", &self.debug_service_type_info);

        debug_struct.finish()
    }
}

impl<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static> WeakTraitService<TComponent, TService>
where TComponent: Unsize<TService>
{
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
            service_phantom_data: Default::default(),
            #[cfg(feature = "debug-type-info")]
            debug_component_type_info: TypeInfo::from_type::<Weak<TComponent>>(),
            #[cfg(feature = "debug-type-info")]
            debug_service_type_info: TypeInfo::from_type::<Weak<TService>>(),
        }
    }
}

impl<TComponent: Sync + Send + 'static, TService:  ?Sized + Sync + Send + 'static> IServiceConstructor for WeakTraitService<TComponent, TService> where TComponent: Unsize<TService>, Self: Sized + Sync + Send {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        let component: Box<Weak<TComponent>> = component.downcast::<Weak<TComponent>>()
            .expect(&format!("Service error, unextected component type [{type_id:?}] [{type_name:?}]", type_id = TypeId::of::<TComponent>(), type_name = type_name::<TComponent>().to_string()));

        let service = Box::into_inner(component) as Weak<TService>;
        return Box::new(service) as Box<dyn Any + Sync + Send>;
    }
}

pub (crate) struct SelfMappingService<TComponent: 'static> {
    component_phantom_data: PhantomData<TComponent>,

    #[cfg(feature = "debug-type-info")]
    debug_type_info: TypeInfo,
}

impl<TComponent: 'static> Debug for SelfMappingService<TComponent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("SelfMappingService");
        debug_struct.field("component_phantom_data", &self.component_phantom_data);

        #[cfg(feature = "debug-type-info")]
        debug_struct.field("debug_type_info", &self.debug_type_info);

        debug_struct.finish()
    }
}

impl<TComponent: 'static> SelfMappingService<TComponent> {
    pub (crate) fn new() -> Self {
        Self {
            component_phantom_data: Default::default(),
            #[cfg(feature = "debug-type-info")]
            debug_type_info: TypeInfo::from_type::<TComponent>()
        }
    }
}

impl<TComponent: Sync + Send + 'static> IServiceConstructor for SelfMappingService<TComponent> {
    fn build(&self, component: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send> {
        return component;
    }
}