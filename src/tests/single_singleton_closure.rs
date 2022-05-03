use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for SingletonDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_singleton_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_closure(|_| Ok(RwLock::new(SingletonDependency { str: "test".to_string() })), DependencyLifeCycle::Singleton).await.unwrap();

    let dependency = root_context.resolve::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());
}

#[test]
fn single_singleton_closure_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_closure_sync(|_| Ok(RwLock::new(SingletonDependency { str: "test".to_string() })), DependencyLifeCycle::Singleton).unwrap();

    let dependency = root_context.resolve_sync::<Arc<RwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());
}

#[tokio::test]
async fn single_singleton_async_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_async_closure(
        |_: crate::DependencyContext| { async move { Ok(RwLock::new(SingletonDependency { str: "test".to_string() }))} },
        DependencyLifeCycle::Singleton
    ).await.unwrap();

    let dependency = root_context.resolve::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());
}

#[test]
fn single_singleton_async_closure_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_async_closure_sync(
        |_: crate::DependencyContext| { async move { Ok(RwLock::new(SingletonDependency { str: "test".to_string() }))} },
        DependencyLifeCycle::Singleton
    ).unwrap();

    let dependency = root_context.resolve_sync::<Arc<RwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());
}