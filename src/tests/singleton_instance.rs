#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[tokio::test]
async fn singleton_instance() {
    use crate::DependencyContext;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    let instance = SingletonDependency { str: "test".to_string() };
    root_context.register_instance(RwLock::new(instance)).await.unwrap();

    let dependency = root_context.resolve::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());

    dependency.write().await.str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.resolve::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency2.read().await.str, "test2".to_string()); // видим измененное состояние в новом объекте
}

#[test]
fn singleton_instance_sync() {
    use crate::DependencyContext;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    let instance = SingletonDependency { str: "test".to_string() };
    root_context.register_instance_sync(RwLock::new(instance)).unwrap();

    let dependency = root_context.resolve_sync::<Arc<RwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency.blocking_read().str, "test".to_string());

    dependency.blocking_write().str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.resolve_sync::<Arc<RwLock<SingletonDependency>>>().unwrap();

    assert_eq!(dependency2.blocking_read().str, "test2".to_string()); // видим измененное состояние в новом объекте
}