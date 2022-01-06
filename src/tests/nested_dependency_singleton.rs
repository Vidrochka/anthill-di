use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct SingletonDependency1 {
    pub str: String,
}

#[async_trait(?Send)]
impl Constructor for SingletonDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[allow(dead_code)]
struct SingletonDependency2 {
    pub d1: Arc<RwLock<SingletonDependency1>>,
    pub d2: Arc<RwLock<SingletonDependency1>>,
}

#[async_trait(?Send)]
impl Constructor for SingletonDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.get_singleton().await?,
            d2: ctx.get_singleton().await?,
        })
    }
}

#[tokio::test]
async fn nested_dependency_singleton() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_singleton::<SingletonDependency1>().await.unwrap();
    root_context.set_singleton::<SingletonDependency2>().await.unwrap();

    let dependency = root_context.get_singleton::<SingletonDependency2>().await.unwrap();

    dependency.read().await.d1.write().await.str = "test2".to_string();

    assert_eq!(dependency.read().await.d2.read().await.str, "test2".to_string());
}