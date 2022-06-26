use std::sync::Arc;

use crate::{
    Constructor,
    types::{
        BuildDependencyResult,
        AnthillRwLock,
    },
};

#[allow(dead_code)]
struct SingletonDependency1 {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency1 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[allow(dead_code)]
struct SingletonDependency2 {
    pub d1: Arc<AnthillRwLock<SingletonDependency1>>,
    pub d2: Arc<AnthillRwLock<SingletonDependency1>>,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency2 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve()?,
            d2: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn nested_dependency_singleton() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<SingletonDependency1>>(LifeCycle::Singleton).unwrap();
    root_context.register_type::<AnthillRwLock<SingletonDependency2>>(LifeCycle::Singleton).unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency2>>>().unwrap();

    dependency.read().unwrap().d1.write().unwrap().str = "test2".to_string();

    assert_eq!(dependency.read().unwrap().d2.read().unwrap().str, "test2".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn nested_dependency_singleton() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<SingletonDependency1>>(LifeCycle::Singleton).await.unwrap();
    root_context.register_type::<AnthillRwLock<SingletonDependency2>>(LifeCycle::Singleton).await.unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency2>>>().await.unwrap();

    dependency.read().await.d1.write().await.str = "test2".to_string();

    assert_eq!(dependency.read().await.d2.read().await.str, "test2".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn nested_dependency_singleton_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<SingletonDependency1>>(LifeCycle::Singleton).unwrap();
    root_context.blocking_register_type::<AnthillRwLock<SingletonDependency2>>(LifeCycle::Singleton).unwrap();

    let dependency = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency2>>>().unwrap();

    dependency.blocking_read().d1.blocking_write().str = "test2".to_string();

    assert_eq!(dependency.blocking_read().d2.blocking_read().str, "test2".to_string());
}