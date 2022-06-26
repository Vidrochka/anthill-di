use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for TransientDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

trait GetStr2: Sync + Send {
    fn get(&self) -> String;
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn transient_service_is_exist() {
    use crate::{DependencyContext, LifeCycle};
    use std::any::TypeId;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).unwrap()
        .map_as::<dyn GetStr>().unwrap();
    
    assert!(root_context.is_service_exist::<Box<dyn GetStr>>());
    assert!(root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn GetStr>>()));
    assert!(!root_context.is_service_exist::<Box<dyn GetStr2>>());
    assert!(!root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn GetStr2>>()));
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn transient_service_is_exist() {
    use crate::{DependencyContext, LifeCycle};
    use std::any::TypeId;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();
    
    assert!(root_context.is_service_exist::<Box<dyn GetStr>>().await);
    assert!(root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn GetStr>>()).await);
    assert!(!root_context.is_service_exist::<Box<dyn GetStr2>>().await);
    assert!(!root_context.is_service_with_type_id_exist(TypeId::of::<Box<dyn GetStr2>>()).await);
}

#[cfg(feature = "blocking")]
#[test]
fn transient_service_is_exist_sync() {
    use crate::{DependencyContext, LifeCycle};
    use std::any::TypeId;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency>(LifeCycle::Transient).unwrap()
        .blocking_map_as::<dyn GetStr>().unwrap();
    
    assert!(root_context.blocking_is_service_exist::<Box<dyn GetStr>>());
    assert!(root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Box<dyn GetStr>>()));
    assert!(!root_context.blocking_is_service_exist::<Box<dyn GetStr2>>());
    assert!(!root_context.blocking_is_service_with_type_id_exist(TypeId::of::<Box<dyn GetStr2>>()));
}