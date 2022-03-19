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
            d2: ctx.get().await?,
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
            d1: Box::new(ctx.get().await?),
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
            d1: Box::new(ctx.get().await?),
        })
    }
}

#[tokio::test]
async fn deep_nested_dependency_cycle_reference() {
    use std::any::{TypeId, type_name};
    use crate::DependencyContext;
    use crate::{
        types::BuildDependencyError,
        extensions::ConstructedDependencySetStrategy
    };
    
    let root_context = DependencyContext::new_root();
    root_context.set_transient::<TransientDependency1>().await.unwrap();
    root_context.set_transient::<TransientDependency2>().await.unwrap();
    root_context.set_transient::<TransientDependency3>().await.unwrap();

    let dependency = root_context.get::<TransientDependency1>().await;

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        child_id: TypeId::of::<TransientDependency1>(),
        child_name: type_name::<TransientDependency1>().to_string(),
        parent_id: TypeId::of::<TransientDependency3>(),
        parent_name: type_name::<TransientDependency3>().to_string(),
    }));
}