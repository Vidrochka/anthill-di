use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "blocking")]
#[test]
fn single_singleton_closure_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_closure(|_| Ok(AnthillRwLock::new(SingletonDependency { str: "test".to_string() })), LifeCycle::Singleton).unwrap();

    let dependency = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_singleton_closure() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_closure(|_| Ok(AnthillRwLock::new(SingletonDependency { str: "test".to_string() })), LifeCycle::Singleton).await.unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_singleton_closure_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_closure(|_| Ok(AnthillRwLock::new(SingletonDependency { str: "test".to_string() })), LifeCycle::Singleton).unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.read().unwrap().str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_singleton_async_closure() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_async_closure(
        |_: crate::DependencyContext| { async move { Ok(AnthillRwLock::new(SingletonDependency { str: "test".to_string() }))} },
        LifeCycle::Singleton
    ).await.unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_singleton_async_closure_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_async_closure(
        |_: crate::DependencyContext| { async move { Ok(AnthillRwLock::new(SingletonDependency { str: "test".to_string() }))} },
        LifeCycle::Singleton
    ).unwrap();

    let dependency = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());
}