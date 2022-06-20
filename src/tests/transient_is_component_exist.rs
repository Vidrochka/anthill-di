

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
}

#[allow(dead_code)]
struct TransientDependency2 {
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


#[cfg(not(feature = "async-mode"))]
#[test]
fn transient_component_is_exist() {
    use crate::{DependencyContext, LifeCycle};
    use std::any::TypeId;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).unwrap();
    
    assert!(root_context.is_component_exist::<TransientDependency>());
    assert!(root_context.is_component_with_type_id_exist(TypeId::of::<TransientDependency>()));
    assert!(!root_context.is_component_exist::<TransientDependency2>());
    assert!(!root_context.is_component_with_type_id_exist(TypeId::of::<TransientDependency2>()));
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn transient_component_is_exist() {
    use crate::{DependencyContext, LifeCycle};
    use std::any::TypeId;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).await.unwrap();
    
    assert!(root_context.is_component_exist::<TransientDependency>().await);
    assert!(root_context.is_component_with_type_id_exist(TypeId::of::<TransientDependency>()).await);
    assert!(!root_context.is_component_exist::<TransientDependency2>().await);
    assert!(!root_context.is_component_with_type_id_exist(TypeId::of::<TransientDependency2>()).await);
}

#[cfg(feature = "blocking")]
#[test]
fn transient_component_is_exist_sync() {
    use crate::{DependencyContext, LifeCycle};
    use std::any::TypeId;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency>(LifeCycle::Transient).unwrap();
    
    assert!(root_context.blocking_is_component_exist::<TransientDependency>());
    assert!(root_context.blocking_is_component_with_type_id_exist(TypeId::of::<TransientDependency>()));
    assert!(!root_context.blocking_is_component_exist::<TransientDependency2>());
    assert!(!root_context.blocking_is_component_with_type_id_exist(TypeId::of::<TransientDependency2>()));
}