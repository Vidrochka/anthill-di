use std::sync::{Weak, Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    Constructor,
    types::BuildDependencyResult,
    DependencyScope
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub s1: Weak<RwLock<ScopedDependency1>>,
    pub t1: TransientDependency2,
}

#[async_trait]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            s1: ctx.get_scoped().await?,
            t1: ctx.get_transient().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub s2: Weak<RwLock<ScopedDependency1>>,
    pub t2: TransientDependency3,
    pub scope: Arc<DependencyScope>,
}

#[async_trait]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        let mut ctx = ctx;
        let scope = ctx.set_empty_scope();

        Ok(Self {
            s2: ctx.get_scoped().await?,
            t2: ctx.get_transient().await?,
            scope: scope,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub s3: Weak<RwLock<ScopedDependency1>>,
}

#[async_trait]
impl Constructor for TransientDependency3 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            s3: ctx.get_scoped().await?,
        })
    }
}

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

#[tokio::test]
async fn deep_nested_dependency_scoped() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_scoped::<RwLock<ScopedDependency1>>().await.unwrap();
    root_context.set_transient::<TransientDependency1>().await.unwrap();
    root_context.set_transient::<TransientDependency2>().await.unwrap();
    root_context.set_transient::<TransientDependency3>().await.unwrap();

    let dependency = root_context.get_transient::<TransientDependency1>().await.unwrap();

    dependency.s1.upgrade().unwrap().write().await.str = "test2".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().read().await.str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().read().await.str, "test".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().read().await.str, "test".to_string());

    dependency.t1.s2.upgrade().unwrap().write().await.str = "test3".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().read().await.str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().read().await.str, "test3".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().read().await.str, "test3".to_string());
}