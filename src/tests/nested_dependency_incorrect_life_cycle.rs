use std::sync::Arc;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};


#[allow(dead_code)]
struct TransientDependency1 {
    pub str: String,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}


#[allow(dead_code)]
struct TransientDependency2 {
    pub d1: Arc<TransientDependency1>,
    pub d2: Arc<TransientDependency1>,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

#[tokio::test]
async fn nested_dependency_incorrect_life_cycle() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency2>().await;

    assert_eq!(dependency.err(), Some(BuildDependencyError::NotFound { type_info: TypeInfo::from_type::<Arc<TransientDependency1>>() }));
}

#[test]
fn nested_dependency_incorrect_life_cycle_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<TransientDependency1>(DependencyLifeCycle::Transient).unwrap();
    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap();

    let dependency = root_context.resolve_sync::<TransientDependency2>();

    assert_eq!(dependency.err(), Some(BuildDependencyError::NotFound { type_info: TypeInfo::from_type::<Arc<TransientDependency1>>() }));
}