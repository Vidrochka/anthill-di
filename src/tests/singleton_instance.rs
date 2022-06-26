#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn singleton_instance() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    let instance = SingletonDependency { str: "test".to_string() };
    root_context.register_instance(AnthillRwLock::new(instance)).unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.read().unwrap().str, "test".to_string());

    dependency.write().unwrap().str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency2.read().unwrap().str, "test2".to_string()); // видим измененное состояние в новом объекте
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn singleton_instance() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    let instance = SingletonDependency { str: "test".to_string() };
    root_context.register_instance(AnthillRwLock::new(instance)).await.unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());

    dependency.write().await.str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.resolve::<Arc<AnthillRwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency2.read().await.str, "test2".to_string()); // видим измененное состояние в новом объекте
}

#[cfg(feature = "blocking")]
#[test]
fn singleton_instance_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    let instance = SingletonDependency { str: "test".to_string() };
    root_context.blocking_register_instance(AnthillRwLock::new(instance)).unwrap();

    let dependency = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());

    dependency.blocking_write().str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.blocking_resolve::<Arc<AnthillRwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency2.blocking_read().str, "test2".to_string()); // видим измененное состояние в новом объекте
}