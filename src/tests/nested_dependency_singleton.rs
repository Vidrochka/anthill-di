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

#[async_trait]
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

#[async_trait]
impl Constructor for SingletonDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

#[tokio::test]
async fn nested_dependency_singleton() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<RwLock<SingletonDependency1>>(DependencyLifeCycle::Singleton).await.unwrap();
    root_context.register_type::<RwLock<SingletonDependency2>>(DependencyLifeCycle::Singleton).await.unwrap();

    let dependency = root_context.resolve::<Arc<RwLock<SingletonDependency2>>>().await.unwrap();

    dependency.read().await.d1.write().await.str = "test2".to_string();

    assert_eq!(dependency.read().await.d2.read().await.str, "test2".to_string());
}