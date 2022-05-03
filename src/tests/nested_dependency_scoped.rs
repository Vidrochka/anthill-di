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
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

#[tokio::test]
async fn nested_dependency_scoped() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<RwLock<ScopedDependency1>>(DependencyLifeCycle::Scoped).await.unwrap();
    root_context.register_type::<RwLock<ScopedDependency2>>(DependencyLifeCycle::Scoped).await.unwrap();

    let dependency = root_context.resolve::<Weak<RwLock<ScopedDependency2>>>().await.unwrap();

    dependency.upgrade().unwrap().read().await.d1.upgrade().unwrap().write().await.str = "test2".to_string();

    assert_eq!(dependency.upgrade().unwrap().read().await.d2.upgrade().unwrap().read().await.str, "test2".to_string());
}

#[test]
fn nested_dependency_scoped_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<RwLock<ScopedDependency1>>(DependencyLifeCycle::Scoped).unwrap();
    root_context.register_type_sync::<RwLock<ScopedDependency2>>(DependencyLifeCycle::Scoped).unwrap();

    let dependency = root_context.resolve_sync::<Weak<RwLock<ScopedDependency2>>>().unwrap();

    dependency.upgrade().unwrap().blocking_read().d1.upgrade().unwrap().blocking_write().str = "test2".to_string();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().d2.upgrade().unwrap().blocking_read().str, "test2".to_string());
}