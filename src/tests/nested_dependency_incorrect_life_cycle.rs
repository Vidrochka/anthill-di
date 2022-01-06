use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};


#[allow(dead_code)]
struct TransientDependency1 {
    pub str: String,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}


#[allow(dead_code)]
struct TransientDependency2 {
    pub d1: Arc<RwLock<TransientDependency1>>,
    pub d2: Arc<RwLock<TransientDependency1>>,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.get_singleton().await?,
            d2: ctx.get_singleton().await?,
        })
    }
}

#[tokio::test]
async fn nested_dependency_incorrect_life_cycle() {
    use std::any::{TypeId, type_name};
    use crate::DependencyContext;
    use crate::{
        types::BuildDependencyError,
        extensions::ConstructedDependencySetStrategy
    };
    
    let root_context = DependencyContext::new_root();
    root_context.set_transient::<TransientDependency1>().await.unwrap();
    root_context.set_transient::<TransientDependency2>().await.unwrap();

    let dependency = root_context.get_transient::<TransientDependency2>().await;

    assert_eq!(dependency.err(), Some(BuildDependencyError::NotFound {
        id: TypeId::of::<Arc<RwLock<TransientDependency1>>>(),
        name: type_name::<Arc<RwLock<TransientDependency1>>>().to_string()
    }));
}