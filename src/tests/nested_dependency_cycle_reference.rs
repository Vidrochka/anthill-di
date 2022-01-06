use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d2: TransientDependency2,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.get_transient().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub d1: Box<TransientDependency1>,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.get_transient().await?),
        })
    }
}

#[tokio::test]
async fn nested_dependency_cycle_reference() {
    use std::any::{TypeId, type_name};
    use crate::DependencyContext;
    use crate::{
        types::BuildDependencyError,
        extensions::ConstructedDependencySetStrategy
    };
    
    let root_context = DependencyContext::new_root();
    root_context.set_transient::<TransientDependency1>().await.unwrap();
    root_context.set_transient::<TransientDependency2>().await.unwrap();

    let dependency = root_context.get_transient::<TransientDependency1>().await;

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        id: TypeId::of::<TransientDependency1>(),
        name: type_name::<TransientDependency1>().to_string(),
        parent_id: TypeId::of::<TransientDependency2>(),
    }));
}