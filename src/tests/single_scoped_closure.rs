use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for ScopedDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_scoped_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Weak;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_closure(|_| Ok(RwLock::new(ScopedDependency { str: "test".to_string() })), DependencyLifeCycle::Scoped).await.unwrap();

    let dependency = root_context.resolve::<Weak<RwLock<ScopedDependency>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());
}

#[tokio::test]
async fn single_scoped_async_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Weak;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_async_closure(
        Box::new(move |_: crate::DependencyContext| {
            Box::pin (async move {
                return Ok(RwLock::new(ScopedDependency { str: "test".to_string() }));
            })
        }),
        DependencyLifeCycle::Scoped
    ).await.unwrap();

    let dependency = root_context.resolve::<Weak<RwLock<ScopedDependency>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());
}