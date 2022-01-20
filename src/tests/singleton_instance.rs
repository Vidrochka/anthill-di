use tokio::sync::RwLock;

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[tokio::test]
async fn singleton_instance() {
    use crate::DependencyContext;

    let root_context = DependencyContext::new_root();
    let instance = SingletonDependency { str: "test".to_string() };
    root_context.add_singleton_instance(RwLock::new(instance)).await.unwrap();

    let dependency = root_context.get_singleton::<RwLock<SingletonDependency>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());

    dependency.write().await.str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.get_singleton::<RwLock<SingletonDependency>>().await.unwrap();

    assert_eq!(dependency2.read().await.str, "test2".to_string()); // видим измененное состояние в новом объекте
}