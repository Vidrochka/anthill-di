#[cfg(feature = "async-mode")]
use crate::constructors::ComponentFromAsyncClosure;
#[cfg(feature = "async-mode")]
use std::future::Future;

use crate::{
    Constructor,
    ComponentFromConstructor,
    types::{
        TypeInfo,
        DeleteComponentResult,
    },
    constructors::{
        ComponentFromClosure,
        ComponentFromInstance,
    },
};
use std::marker::Unsize;
use std::{
    any::TypeId,
    sync::Arc,
};

use crate::{
    CoreContext,
    LocalContext,
    ServiceMappingBuilder,
    types::{
        BuildDependencyResult,
        AddDependencyResult,
        MapComponentResult,
    },
    LifeCycle,
};

#[derive(Debug, PartialEq, Clone)]
pub (crate) enum DependencyContextId {
    TypeId(TypeInfo),
    Root,
}

/// Root di context, or child context (in ctor or closure)
/// Main component. You work with it most of the time
#[derive(Debug, Clone)]
pub struct DependencyContext {
    id: DependencyContextId,
    core_context: Arc<CoreContext>,
    pub (crate) local_context: Arc<LocalContext>,
}

impl DependencyContext {
    /// Create new root empty context
    pub fn new_root() -> Self {
        Self {
            id: DependencyContextId::Root,
            core_context: Arc::new(Default::default()),
            local_context: Arc::new(Default::default()),
        }
    }

    #[inline(always)]
    pub (crate) fn new_dependency(id: DependencyContextId, core_context: Arc<CoreContext>, local_context: Arc<LocalContext>) -> Self {
        Self { id, core_context, local_context, }
    }

    /// Set saved local context
    #[inline(always)]
    pub fn set_context(&mut self, local_context: Arc<LocalContext>) { self.local_context = local_context }

    /// Set new local context and return copy
    #[inline(always)]
    pub fn set_empty_context(&mut self) -> Arc<LocalContext> {
        self.local_context = Arc::new(Default::default());
        self.local_context.clone()
    }

    /// Get copy local context.
    /// 
    /// Save local context, before set_empty_context, if you need not to lose context dependent components
    /// 
    /// Then you can restore context with ```set_context```
    #[inline(always)]
    pub fn get_context(&self) -> Arc<LocalContext> { self.local_context.clone() }
}

#[cfg(feature = "async-mode")]
impl DependencyContext {
    /// Register component witch implement trait Constructor
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_type::<SomeComponent>(LifeCycle::Transient).await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_type::<SomeComponent>(LifeCycle::Transient).await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn register_type<TComponent: Constructor + Sync + Send + 'static>(&self, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        self.core_context.register::<TComponent>(Box::new(ComponentFromConstructor::<TComponent>::new()), life_cycle).await
    }

    /// Register component from async closure
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_closure(async |_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_closure(async |_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn register_async_closure<TComponent, TFuture, TClosure>(&self, closure: TClosure, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>>
    where
        TComponent: Sync + Send + 'static,
        TFuture: Future<Output = BuildDependencyResult<TComponent>>,
        TFuture: Sync + Send + 'static,
        TClosure: Fn(DependencyContext) -> TFuture,
        TClosure: Sync + Send + 'static,
    {
        self.core_context.register::<TComponent>(Box::new(ComponentFromAsyncClosure::<TComponent, TFuture, TClosure>::new(closure)), life_cycle).await
    }

    /// Register component from closure
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_closure(|_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_closure(|_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn register_closure<TComponent: Sync + Send + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send + 'static>(&self, closure: TClosure, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        self.core_context.register::<TComponent>(Box::new(ComponentFromClosure::<TComponent>::new(Box::new(closure))), life_cycle).await
    }

    /// Register component instance as singleton
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_instance::<SomeComponent>(SomeComponent::new()).await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_instance::<SomeComponent>(SomeComponent::new()).await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn register_instance<TComponent: Sync + Send + 'static>(&self, instance: TComponent) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        self.core_context.register::<TComponent>(Box::new(ComponentFromInstance::new(instance)), LifeCycle::Singleton).await
    }

    /// Map component as service
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// //...
    /// 
    /// // SomeComponent must be registered, or will be error
    /// root_context.map_component::<SomeComponent, dyn SomeService>().await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // SomeComponent must be registered, or will be error
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.map_component::<SomeComponent, dyn SomeService>().await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&self) -> MapComponentResult<Arc<CoreContext>> where TComponent: Unsize<TService> {
        self.core_context.map_component::<TComponent, TService>().await
    }

    /// Resolve first component, mapped to service
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.resolve::<Box<dyn Service>>().await.unwrap()
    /// ctx.resolve::<Box<Component>>().await.unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.resolve::<Arc<dyn Service>>().await.unwrap()
    /// ctx.resolve::<Arc<Component>>().await.unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.resolve::<Weak<dyn Service>>().await.unwrap()
    /// ctx.resolve::<Weak<Component>>().await.unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.resolve::<Weak<RwLock<dyn Service>>>().await.unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let service: Box<dyn SomeTrait> = root_context.resolve::<Box<dyn SomeService>>().await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let service: Box<dyn SomeTrait> = ctx.resolve::<Box<dyn SomeService>>().await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn resolve<'a, TService: Sync + Send + 'static>(&'a self) -> BuildDependencyResult<TService> {
        self.core_context.resolve::<TService>(self.id.clone(), self.local_context.clone()).await
    }

    /// Resolve component with TypeId, mapped to service
    /// That may be helpful in case, when you need current component, type is lost, but you can save TypeId as variable
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.resolve_by_type_id::<Box<dyn Service>>(TypeId::of::<Component>()).await.unwrap()
    /// ctx.resolve_by_type_id::<Box<Component>>(TypeId::of::<Component>()).await.unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.resolve_by_type_id::<Arc<dyn Service>>(TypeId::of::<Component>()).await.unwrap()
    /// ctx.resolve_by_type_id::<Arc<Component>>(TypeId::of::<Component>()).await.unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.resolve_by_type_id::<Weak<dyn Service>>(TypeId::of::<Component>()).await.unwrap()
    /// ctx.resolve_by_type_id::<Weak<Component>>(TypeId::of::<Component>()).await.unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.resolve_by_type_id::<Weak<RwLock<dyn Service>>>(TypeId::of::<Component>()).await.unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let service: Box<dyn SomeTrait> = root_context.resolve_by_type_id::<Box<dyn SomeService>>(TypeId::of::<SomeComponent>()).await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let service: Box<dyn SomeTrait> = ctx.resolve_by_type_id::<Box<dyn SomeService>>(TypeId::of::<SomeComponent>()).await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn resolve_by_type_id<TService: Sync + Send + 'static>(&self, component_type_id: TypeId) -> BuildDependencyResult<TService> {
        self.core_context.resolve_by_type_id::<TService>(component_type_id, self.id.clone(), self.local_context.clone()).await
    }

    /// Resolve all component, mapped to service
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.resolve_collection::<Box<dyn Service>>().await.unwrap()
    /// ctx.resolve_collection::<Box<Component>>().await.unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.resolve_collection::<Arc<dyn Service>>().await.unwrap()
    /// ctx.resolve_collection::<Arc<Component>>().await.unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.resolve_collection::<Weak<dyn Service>>().await.unwrap()
    /// ctx.resolve_collection::<Weak<Component>>().await.unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.resolve_collection::<Weak<RwLock<dyn Service>>>().await.unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let collection: Vec<Box<dyn SomeService>> = root_context.resolve_collection::<Box<dyn SomeService>>().await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let collection: Vec<Box<dyn SomeService>> = ctx.resolve_collection::<Box<dyn SomeService>>().await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn resolve_collection<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<Vec<TService>> {
        self.core_context.resolve_collection::<TService>(self.id.clone(), self.local_context.clone()).await
    }

    /// Delete component
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.delete_component::<SomeComponent>().await.unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.delete_component::<SomeComponent>().await.unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn delete_component<TComponent: 'static>(&self) -> DeleteComponentResult<()> {
        self.core_context.delete_component::<TComponent>().await
    }

    /// Check service existence
    /// ``` ignore
    /// // You can check transient like:
    /// ctx.is_service_exist::<Box<dyn Service>>().await.unwrap()
    /// ctx.is_service_exist::<Box<Component>>().await.unwrap()
    /// 
    /// // You can check singleton like:
    /// ctx.is_service_exist::<Arc<dyn Service>>().await.unwrap()
    /// ctx.is_service_exist::<Arc<Component>>().await.unwrap()
    /// 
    /// // You can check context dependent like:
    /// ctx.is_service_exist::<Weak<dyn Service>>().await.unwrap()
    /// ctx.is_service_exist::<Weak<Component>>().await.unwrap()
    /// 
    /// // remember, if you register as RwLock<Component>, map as RwLock<dyn Service>, check like:
    /// ctx.is_service_exist::<Weak<RwLock<dyn Service>>>().await.unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_service_exist::<Box<dyn SomeService>>().await;
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_service_exist::<Box<dyn SomeService>>().await;
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn is_service_exist<TService: 'static>(&self) -> bool {
        self.core_context.is_service_exist(TypeId::of::<TService>()).await
    }

    /// Check service existence by type id
    /// ``` ignore
    /// // You can check transient like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Box<dyn Service>>()).await.unwrap()
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Box<Component>>()).await.unwrap()
    /// 
    /// // You can check singleton like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Arc<dyn Service>>()).await.unwrap()
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Arc<Component>>()).await.unwrap()
    /// 
    /// // You can check context dependent like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Weak<dyn Service>>()).await.unwrap()
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Weak<Component>>()).await.unwrap()
    /// 
    /// // remember, if you register as RwLock<Component>, map as RwLock<dyn Service>, check like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Weak<RwLock<dyn Service>>>()).await.unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeService>>()).await;
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeService>>()).await;
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn is_service_with_type_id_exist(&self, service_type_id: TypeId) -> bool {
        self.core_context.is_service_exist(service_type_id).await
    }

    /// Check component existence
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_component_exist::<SomeComponent>().await;
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_component_exist::<SomeComponent>().await;
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn is_component_exist<TComponent: 'static>(&self) -> bool {
        self.core_context.is_component_exist(TypeId::of::<TComponent>()).await
    }

    /// Check component existence by type id
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_component_with_type_id_exist(TypeId::of::<SomeComponent>()).await;
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_component_with_type_id_exist(TypeId::of::<SomeComponent>()).await;
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub async fn is_component_with_type_id_exist(&self, component_type_id: TypeId) -> bool {
        self.core_context.is_component_exist(component_type_id).await
    }
}

#[cfg(feature = "blocking")]
impl DependencyContext {
    /// Register component witch implement trait Constructor (blocking version)
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.blocking_register_type::<SomeComponent>(LifeCycle::Transient).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.blocking_register_type::<SomeComponent>(LifeCycle::Transient).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_register_type<TComponent: Constructor + Sync + Send + 'static>(&self, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_register::<TComponent>(Box::new(ComponentFromConstructor::<TComponent>::new()), life_cycle)
        }).join().unwrap()
    }

    /// Register component from async closure (blocking version)
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.blocking_register_async_closure(async |_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.blocking_register_async_closure(async |_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_register_async_closure<TComponent, TFuture, TClosure>(&self, closure: TClosure, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>>
    where
        TComponent: Sync + Send + 'static,
        TFuture: Future<Output = BuildDependencyResult<TComponent>>,
        TFuture: Sync + Send + 'static,
        TClosure: Fn(DependencyContext) -> TFuture,
        TClosure: Sync + Send + 'static,
    {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_register::<TComponent>(Box::new(ComponentFromAsyncClosure::<TComponent, TFuture, TClosure>::new(closure)), life_cycle)
        }).join().unwrap()
    }

    /// Register component from closure (blocking version)
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.blocking_register_closure(|_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.blocking_register_closure(|_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_register_closure<TComponent: Sync + Send + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send + 'static>(&self, closure: TClosure, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_register::<TComponent>(Box::new(ComponentFromClosure::<TComponent>::new(Box::new(closure))), life_cycle)
        }).join().unwrap()
    }

    /// Register component instance as singleton (blocking version)
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.blocking_register_instance::<SomeComponent>(SomeComponent::new()).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.blocking_register_instance::<SomeComponent>(SomeComponent::new()).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_register_instance<TComponent: Sync + Send + 'static>(&self, instance: TComponent) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_register::<TComponent>(Box::new(ComponentFromInstance::new(instance)), LifeCycle::Singleton)
        }).join().unwrap()
    }

    /// Map component as service (blocking version)
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// //...
    /// 
    /// // SomeComponent must be registered, or will be error
    /// root_context.blocking_map_component::<SomeComponent, dyn SomeService>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // SomeComponent must be registered, or will be error
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.blocking_map_component::<SomeComponent, dyn SomeService>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&self) -> MapComponentResult<Arc<CoreContext>> where TComponent: Unsize<TService> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_map_component::<TComponent, TService>()
        }).join().unwrap()
    }

    /// Resolve first component, mapped to service (blocking version)
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.blocking_resolve::<Box<dyn Service>>().unwrap()
    /// ctx.blocking_resolve::<Box<Component>>().unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.blocking_resolve::<Arc<dyn Service>>().unwrap()
    /// ctx.blocking_resolve::<Arc<Component>>().unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.blocking_resolve::<Weak<dyn Service>>().unwrap()
    /// ctx.blocking_resolve::<Weak<Component>>().unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.blocking_resolve::<Weak<RwLock<dyn Service>>>().unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let service: Box<dyn SomeTrait> = root_context.blocking_resolve::<Box<dyn SomeService>>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let service: Box<dyn SomeTrait> = ctx.blocking_resolve::<Box<dyn SomeService>>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_resolve<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<TService> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_resolve::<TService>(self_copy.id.clone(), self_copy.local_context.clone())
        }).join().unwrap()
    }

    /// Resolve component with TypeId, mapped to service (blocking version)
    /// That may be helpful in case, when you need current component, type is lost, but you can save TypeId as variable
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.blocking_resolve_by_type_id::<Box<dyn Service>>(TypeId::of::<Component>()).unwrap()
    /// ctx.blocking_resolve_by_type_id::<Box<Component>>(TypeId::of::<Component>()).unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.blocking_resolve_by_type_id::<Arc<dyn Service>>(TypeId::of::<Component>()).unwrap()
    /// ctx.blocking_resolve_by_type_id::<Arc<Component>>(TypeId::of::<Component>()).unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.blocking_resolve_by_type_id::<Weak<dyn Service>>(TypeId::of::<Component>()).unwrap()
    /// ctx.blocking_resolve_by_type_id::<Weak<Component>>(TypeId::of::<Component>()).unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.blocking_resolve_by_type_id::<Weak<RwLock<dyn Service>>>(TypeId::of::<Component>()).unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let service: Box<dyn SomeTrait> = root_context.blocking_resolve_by_type_id::<Box<dyn SomeService>>(TypeId::of::<SomeComponent>()).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let service: Box<dyn SomeTrait> = ctx.blocking_resolve_by_type_id::<Box<dyn SomeService>>(TypeId::of::<SomeComponent>()).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_resolve_by_type_id<TService: Sync + Send + 'static>(&self, component_type_id: TypeId) -> BuildDependencyResult<TService> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_resolve_by_type_id::<TService>(component_type_id, self_copy.id.clone(), self_copy.local_context.clone())
        }).join().unwrap()
    }

    /// Resolve all component, mapped to service (blocking version)
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.blocking_resolve_collection::<Box<dyn Service>>().unwrap()
    /// ctx.blocking_resolve_collection::<Box<Component>>().unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.blocking_resolve_collection::<Arc<dyn Service>>().unwrap()
    /// ctx.blocking_resolve_collection::<Arc<Component>>().unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.blocking_resolve_collection::<Weak<dyn Service>>().unwrap()
    /// ctx.blocking_resolve_collection::<Weak<Component>>().unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.blocking_resolve_collection::<Weak<RwLock<dyn Service>>>().unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let collection: Vec<Box<dyn SomeService>> = root_context.blocking_resolve_collection::<Box<dyn SomeService>>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let collection: Vec<Box<dyn SomeService>> = ctx.blocking_resolve_collection::<Box<dyn SomeService>>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_resolve_collection<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<Vec<TService>> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_resolve_collection::<TService>(self_copy.id.clone(), self_copy.local_context.clone())
        }).join().unwrap()
    }

    /// Delete component (blocking version)
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.blocking_delete_component::<SomeComponent>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.blocking_delete_component::<SomeComponent>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_delete_component<TComponent: 'static>(&self) -> DeleteComponentResult<()> {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_delete_component::<TComponent>()
        }).join().unwrap()
    }

    /// Check service existence (blocking version)
    /// ``` ignore
    /// // You can check transient like:
    /// ctx.blocking_is_service_exist::<Box<dyn Service>>().unwrap()
    /// ctx.blocking_is_service_exist::<Box<Component>>().unwrap()
    /// 
    /// // You can check singleton like:
    /// ctx.blocking_is_service_exist::<Arc<dyn Service>>().unwrap()
    /// ctx.blocking_is_service_exist::<Arc<Component>>().unwrap()
    /// 
    /// // You can check context dependent like:
    /// ctx.blocking_is_service_exist::<Weak<dyn Service>>().unwrap()
    /// ctx.blocking_is_service_exist::<Weak<Component>>().unwrap()
    /// 
    /// // remember, if you register as RwLock<Component>, map as RwLock<dyn Service>, check like:
    /// ctx.blocking_is_service_exist::<Weak<RwLock<dyn Service>>>().unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.blocking_is_service_exist::<Box<dyn SomeService>>();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.blocking_is_service_exist::<Box<dyn SomeService>>();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_is_service_exist<TService: 'static>(&self) -> bool {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_is_service_exist(TypeId::of::<TService>())
        }).join().unwrap()
    }

    /// Check service existence by type id (blocking version)
    /// ``` ignore
    /// // You can check transient like:
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Box<dyn Service>>()).unwrap()
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Box<Component>>()).unwrap()
    /// 
    /// // You can check singleton like:
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Arc<dyn Service>>()).unwrap()
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Arc<Component>>()).unwrap()
    /// 
    /// // You can check context dependent like:
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Weak<dyn Service>>()).unwrap()
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Weak<Component>>()).unwrap()
    /// 
    /// // remember, if you register as RwLock<Component>, map as RwLock<dyn Service>, check like:
    /// ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Weak<RwLock<dyn Service>>>()).unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeService>>());
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.blocking_is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeService>>());
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_is_service_with_type_id_exist(&self, service_type_id: TypeId) -> bool {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_is_service_exist(service_type_id)
        }).join().unwrap()
    }

    /// Check component existence (blocking version)
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.blocking_is_component_exist::<SomeComponent>();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.blocking_is_component_exist::<SomeComponent>();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_is_component_exist<TComponent: 'static>(&self) -> bool {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_is_component_exist(TypeId::of::<TComponent>())
        }).join().unwrap()
    }

    /// Check component existence by type id (blocking version)
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.blocking_is_component_with_type_id_exist(TypeId::of::<SomeComponent>());
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// #[async_trait_with_sync::async_trait(Sync)]
    /// impl Constructor for SomeOtherComponent {
    ///     async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.blocking_is_component_with_type_id_exist(TypeId::of::<SomeComponent>());
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn blocking_is_component_with_type_id_exist(&self, component_type_id: TypeId) -> bool {
        let self_copy = self.clone();
        std::thread::spawn(move || {
            self_copy.core_context.blocking_is_component_exist(component_type_id)
        }).join().unwrap()
    }
}

#[cfg(not(feature = "async-mode"))]
impl DependencyContext {
    /// Register component witch implement trait Constructor
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_type::<SomeComponent>(LifeCycle::Transient).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_type::<SomeComponent>(LifeCycle::Transient).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn register_type<TComponent: Constructor + Sync + Send + 'static>(&self, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        self.core_context.register::<TComponent>(Box::new(ComponentFromConstructor::<TComponent>::new()), life_cycle)
    }

    /// Register component from closure
    /// 
    /// Remember, lifetime required smart pointers wrapper:
    /// * if register Component as Transient, resolve as Component or Box<dyn Service>
    /// * if register Component as Singleton, resolve as Arc<Component> or Arc<dyn Service>
    /// * if register Component as ContextDependent, resolve as Weak<Component> or Weak<dyn Service>
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_closure(|_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_closure(|_ctx| Ok(SomeComponent::new()), LifeCycle::Transient).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn register_closure<TComponent: Sync + Send + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TComponent> + Sync + Send + 'static>(&self, closure: TClosure, life_cycle: LifeCycle) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        self.core_context.register::<TComponent>(Box::new(ComponentFromClosure::<TComponent>::new(Box::new(closure))), life_cycle)
    }

    /// Register component instance as singleton
    /// 
    /// Remember, for mutable Singleton/ContextDependent instance, you need register component, wrapped in RwLock<T> 
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.register_instance::<SomeComponent>(SomeComponent::new()).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.register_instance::<SomeComponent>(SomeComponent::new()).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn register_instance<TComponent: Sync + Send + 'static>(&self, instance: TComponent) -> AddDependencyResult<ServiceMappingBuilder<TComponent>> {
        self.core_context.register::<TComponent>(Box::new(ComponentFromInstance::new(instance)), LifeCycle::Singleton)
    }

    /// Map component as service
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// //...
    /// 
    /// // SomeComponent must be registered, or will be error
    /// root_context.map_component::<SomeComponent, dyn SomeService>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // SomeComponent must be registered, or will be error
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.map_component::<SomeComponent, dyn SomeService>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn map_component<TComponent: Sync + Send + 'static, TService: ?Sized + Sync + Send + 'static>(&self) -> MapComponentResult<Arc<CoreContext>> where TComponent: Unsize<TService> {
        self.core_context.map_component::<TComponent, TService>()
    }

    /// Resolve first component, mapped to service
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.resolve::<Box<dyn Service>>().unwrap()
    /// ctx.resolve::<Box<Component>>().unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.resolve::<Arc<dyn Service>>().unwrap()
    /// ctx.resolve::<Arc<Component>>().unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.resolve::<Weak<dyn Service>>().unwrap()
    /// ctx.resolve::<Weak<Component>>().unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.resolve::<Weak<RwLock<dyn Service>>>().unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let service: Box<dyn SomeTrait> = root_context.resolve::<Box<dyn SomeService>>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let service: Box<dyn SomeTrait> = ctx.resolve::<Box<dyn SomeService>>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn resolve<'a, TService: Sync + Send + 'static>(&'a self) -> BuildDependencyResult<TService> {
        self.core_context.resolve::<TService>(self.id.clone(), self.local_context.clone())
    }

    /// Resolve component with TypeId, mapped to service
    /// That may be helpful in case, when you need current component, type is lost, but you can save TypeId as variable
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.resolve_by_type_id::<Box<dyn Service>>(TypeId::of::<Component>()).unwrap()
    /// ctx.resolve_by_type_id::<Box<Component>>(TypeId::of::<Component>()).unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.resolve_by_type_id::<Arc<dyn Service>>(TypeId::of::<Component>()).unwrap()
    /// ctx.resolve_by_type_id::<Arc<Component>>(TypeId::of::<Component>()).unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.resolve_by_type_id::<Weak<dyn Service>>(TypeId::of::<Component>()).unwrap()
    /// ctx.resolve_by_type_id::<Weak<Component>>(TypeId::of::<Component>()).unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.resolve_by_type_id::<Weak<RwLock<dyn Service>>>(TypeId::of::<Component>()).unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let service: Box<dyn SomeTrait> = root_context.resolve_by_type_id::<Box<dyn SomeService>>(TypeId::of::<SomeComponent>()).unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// trait SomeService {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let service: Box<dyn SomeTrait> = ctx.resolve_by_type_id::<Box<dyn SomeService>>(TypeId::of::<SomeComponent>()).unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn resolve_by_type_id<TService: Sync + Send + 'static>(&self, component_type_id: TypeId) -> BuildDependencyResult<TService> {
        self.core_context.resolve_by_type_id::<TService>(component_type_id, self.id.clone(), self.local_context.clone())
    }

    /// Resolve all component, mapped to service
    /// ``` ignore
    /// // You can resolve transient like:
    /// ctx.resolve_collection::<Box<dyn Service>>().unwrap()
    /// ctx.resolve_collection::<Box<Component>>().unwrap()
    /// 
    /// // You can resolve singleton like:
    /// ctx.resolve_collection::<Arc<dyn Service>>().unwrap()
    /// ctx.resolve_collection::<Arc<Component>>().unwrap()
    /// 
    /// // You can resolve context dependent like:
    /// ctx.resolve_collection::<Weak<dyn Service>>().unwrap()
    /// ctx.resolve_collection::<Weak<Component>>().unwrap()
    /// 
    /// // for mutable Arc and Weak, register as RwLock<Component>, map as RwLock<dyn Service>, and resolve like:
    /// ctx.resolve_collection::<Weak<RwLock<dyn Service>>>().unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let collection: Vec<Box<dyn SomeService>> = root_context.resolve_collection::<Box<dyn SomeService>>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         let collection: Vec<Box<dyn SomeService>> = ctx.resolve_collection::<Box<dyn SomeService>>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn resolve_collection<TService: Sync + Send + 'static>(&self) -> BuildDependencyResult<Vec<TService>> {
        self.core_context.resolve_collection::<TService>(self.id.clone(), self.local_context.clone())
    }

    /// Delete component
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// root_context.delete_component::<SomeComponent>().unwrap();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         // unwrap or map error to BuildDependencyError
    ///         ctx.delete_component::<SomeComponent>().unwrap();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn delete_component<TComponent: 'static>(&self) -> DeleteComponentResult<()> {
        self.core_context.delete_component::<TComponent>()
    }

    /// Check service existence
    /// ``` ignore
    /// // You can check transient like:
    /// ctx.is_service_exist::<Box<dyn Service>>().unwrap()
    /// ctx.is_service_exist::<Box<Component>>().unwrap()
    /// 
    /// // You can check singleton like:
    /// ctx.is_service_exist::<Arc<dyn Service>>().unwrap()
    /// ctx.is_service_exist::<Arc<Component>>().unwrap()
    /// 
    /// // You can check context dependent like:
    /// ctx.is_service_exist::<Weak<dyn Service>>().unwrap()
    /// ctx.is_service_exist::<Weak<Component>>().unwrap()
    /// 
    /// // remember, if you register as RwLock<Component>, map as RwLock<dyn Service>, check like:
    /// ctx.is_service_exist::<Weak<RwLock<dyn Service>>>().unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_service_exist::<Box<dyn SomeService>>();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_service_exist::<Box<dyn SomeService>>();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn is_service_exist<TService: 'static>(&self) -> bool {
        self.core_context.is_service_exist(TypeId::of::<TService>())
    }

    /// Check service existence by type id
    /// ``` ignore
    /// // You can check transient like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Box<dyn Service>>()).unwrap()
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Box<Component>>()).unwrap()
    /// 
    /// // You can check singleton like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Arc<dyn Service>>()).unwrap()
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Arc<Component>>()).unwrap()
    /// 
    /// // You can check context dependent like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Weak<dyn Service>>()).unwrap()
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Weak<Component>>()).unwrap()
    /// 
    /// // remember, if you register as RwLock<Component>, map as RwLock<dyn Service>, check like:
    /// ctx.is_service_with_type_id_exist(TypeId::of::<Weak<RwLock<dyn Service>>>()).unwrap()
    ///  
    /// ```
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeService>>());
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// trait SomeService {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_service_with_type_id_exist(TypeId::of::<Box<dyn SomeService>>());
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn is_service_with_type_id_exist(&self, service_type_id: TypeId) -> bool {
        self.core_context.is_service_exist(service_type_id)
    }

    /// Check component existence
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_component_exist::<SomeComponent>();
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_component_exist::<SomeComponent>();
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn is_component_exist<TComponent: 'static>(&self) -> bool {
        self.core_context.is_component_exist(TypeId::of::<TComponent>())
    }

    /// Check component existence by type id
    ///# Example
    ///---
    /// From root context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// let root_context = DependencyContext::new_root();
    /// let is_exist: bool = root_context.is_component_with_type_id_exist(TypeId::of::<SomeComponent>());
    /// ```
    /// 
    /// ---
    /// 
    /// From ctor context
    /// ```ignore
    /// struct SomeComponent {}
    /// 
    /// impl Constructor for SomeOtherComponent {
    ///     fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
    ///         let is_exist: bool = ctx.is_component_with_type_id_exist(TypeId::of::<SomeComponent>());
    ///         Ok(Self { })
    ///     }
    /// }
    /// 
    /// ```
    #[inline(always)]
    pub fn is_component_with_type_id_exist(&self, component_type_id: TypeId) -> bool {
        self.core_context.is_component_exist(component_type_id)
    }
}