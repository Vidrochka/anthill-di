use crate::{Constructor, types::BuildDependencyResult};

#[allow(dead_code)]
struct ContextDependentDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[allow(dead_code)]
struct ContextDependentDependency2 {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency2 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn context_dependent_is_exist() {
    use crate::{
        DependencyContext,
        LifeCycle
    };
    use std::{
        any::TypeId,
        sync::Weak
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<ContextDependentDependency>(LifeCycle::ContextDependent).unwrap();

    assert!(root_context.is_service_exist::<Weak<ContextDependentDependency>>());
    assert!(root_context.is_service_with_type_id_exist(TypeId::of::<Weak<ContextDependentDependency>>()));
    assert!(!root_context.is_service_exist::<Weak<ContextDependentDependency2>>());
    assert!(!root_context.is_service_with_type_id_exist(TypeId::of::<Weak<ContextDependentDependency2>>()));
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn context_dependent_is_exist() {
    use crate::{
        DependencyContext,
        LifeCycle
    };
    use std::{
        any::TypeId,
        sync::Weak
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<ContextDependentDependency>(LifeCycle::ContextDependent).await.unwrap();

    assert!(root_context.is_service_exist::<Weak<ContextDependentDependency>>().await);
    assert!(root_context.is_service_with_type_id_exist(TypeId::of::<Weak<ContextDependentDependency>>()).await);
    assert!(!root_context.is_service_exist::<Weak<ContextDependentDependency2>>().await);
    assert!(!root_context.is_service_with_type_id_exist(TypeId::of::<Weak<ContextDependentDependency2>>()).await);
}

#[cfg(feature = "blocking")]
#[test]
fn context_dependent_is_exist_sync() {
    use crate::DependencyContext;
    use crate::LifeCycle;
    use std::{
        sync::Weak,
        any::TypeId
    };

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<ContextDependentDependency>(LifeCycle::ContextDependent).unwrap();

    assert!(root_context.blocking_is_service_exist::<Weak<ContextDependentDependency>>());
    assert!(root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Weak<ContextDependentDependency>>()));
    assert!(!root_context.blocking_is_service_exist::<Weak<ContextDependentDependency2>>());
    assert!(!root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Weak<ContextDependentDependency2>>()));
}