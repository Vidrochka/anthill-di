use crate::{Constructor, types::BuildDependencyResult};

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

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_singleton() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<SingletonDependency>>(LifeCycle::Singleton).unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.read().unwrap().str, "test".to_string());

    dependency.write().unwrap().str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency2.read().unwrap().str, "test2".to_string()); // видим измененное состояние в новом объекте
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_singleton() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<SingletonDependency>>(LifeCycle::Singleton).await.unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());

    dependency.write().await.str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency2.read().await.str, "test2".to_string()); // видим измененное состояние в новом объекте
}

#[cfg(feature = "blocking")]
#[test]
fn single_singleton_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<SingletonDependency>>(LifeCycle::Singleton).unwrap();

    let dependency = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());

    dependency.blocking_write().str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency2.blocking_read().str, "test2".to_string()); // видим измененное состояние в новом объекте
}