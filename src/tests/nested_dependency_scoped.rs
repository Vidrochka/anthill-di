use std::sync::Weak;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct ScopedDependency1 {
    pub str: String,
}

#[async_trait]
impl Constructor for ScopedDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[allow(dead_code)]
struct ScopedDependency2 {
    pub d1: Weak<RwLock<ScopedDependency1>>,
    pub d2: Weak<RwLock<ScopedDependency1>>,
}

#[async_trait]
impl Constructor for ScopedDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.get_scoped().await?,
            d2: ctx.get_scoped().await?,
        })
    }
}

#[tokio::test]
async fn nested_dependency_scoped() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_scoped::<RwLock<ScopedDependency1>>().await.unwrap();
    root_context.set_scoped::<RwLock<ScopedDependency2>>().await.unwrap();

    let dependency = root_context.get_scoped::<RwLock<ScopedDependency2>>().await.unwrap();

    dependency.upgrade().unwrap().read().await.d1.upgrade().unwrap().write().await.str = "test2".to_string();

    assert_eq!(dependency.upgrade().unwrap().read().await.d2.upgrade().unwrap().read().await.str, "test2".to_string());
}