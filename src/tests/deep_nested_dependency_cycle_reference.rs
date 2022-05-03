use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d2: TransientDependency2,
}

#[async_trait]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub d1: Box<TransientDependency3>,
}

#[async_trait]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.resolve().await?),
        })
    }
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub d1: Box<TransientDependency1>,
}

#[async_trait]
impl Constructor for TransientDependency3 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.resolve().await?),
        })
    }
}

#[tokio::test]
async fn deep_nested_dependency_cycle_reference() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency3>(DependencyLifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await;

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        child_type_info: TypeInfo::from_type::<TransientDependency1>(),
        parent_type_info: TypeInfo::from_type::<TransientDependency3>(),
    }));
}

#[test]
fn deep_nested_dependency_cycle_reference_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<TransientDependency1>(DependencyLifeCycle::Transient).unwrap();
    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap();
    root_context.register_type_sync::<TransientDependency3>(DependencyLifeCycle::Transient).unwrap();

    let dependency = root_context.resolve_sync::<TransientDependency1>();

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        child_type_info: TypeInfo::from_type::<TransientDependency1>(),
        parent_type_info: TypeInfo::from_type::<TransientDependency3>(),
    }));
}