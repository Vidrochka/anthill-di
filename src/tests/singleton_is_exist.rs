use crate::{Constructor, types::BuildDependencyResult};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[allow(dead_code)]
struct SingletonDependency2 {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency2 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_transient() {
    use crate::DependencyContext;
    use crate::LifeCycle;
    use std::{
        any::TypeId,
        sync::Arc
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).unwrap();

    assert!(root_context.is_service_exist::<Arc<SingletonDependency>>());
    assert!(root_context.is_service_with_type_id_exist(TypeId::of::<Arc<SingletonDependency>>()));
    assert!(!root_context.is_service_exist::<Arc<SingletonDependency2>>());
    assert!(!root_context.is_service_with_type_id_exist(TypeId::of::<Arc<SingletonDependency2>>()));
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_transient() {
    use crate::DependencyContext;
    use crate::LifeCycle;
    use std::{
        any::TypeId,
        sync::Arc
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).await.unwrap();

    assert!(root_context.is_service_exist::<Arc<SingletonDependency>>().await);
    assert!(root_context.is_service_with_type_id_exist(TypeId::of::<Arc<SingletonDependency>>()).await);
    assert!(!root_context.is_service_exist::<Arc<SingletonDependency2>>().await);
    assert!(!root_context.is_service_with_type_id_exist(TypeId::of::<Arc<SingletonDependency2>>()).await);
}

#[cfg(feature = "blocking")]
#[test]
fn singleton_is_exist_sync() {
    use crate::DependencyContext;
    use crate::LifeCycle;
    use std::{
        sync::Arc,
        any::TypeId
    };

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<SingletonDependency>(LifeCycle::Singleton).unwrap();

    assert!(root_context.blocking_is_service_exist::<Arc<SingletonDependency>>());
    assert!(root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Arc<SingletonDependency>>()));
    assert!(!root_context.blocking_is_service_exist::<Arc<SingletonDependency2>>());
    assert!(!root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Arc<SingletonDependency2>>()));
}