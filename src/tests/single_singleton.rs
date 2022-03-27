use async_trait::async_trait;

use crate::{Constructor, types::BuildDependencyResult};

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
async fn single_singleton() {
    use crate::{DependencyContext, DependencyLifeCycle};
    //use crate::extensions::ConstructedDependencySetStrategy;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    //root_context.set_singleton::<RwLock<SingletonDependency>>().await.unwrap();
    root_context.register::<RwLock<SingletonDependency>>(DependencyLifeCycle::Singleton).await.unwrap();
    println!("{root_context:#?}");

    let dependency = root_context.get::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());

    dependency.write().await.str = "test2".to_string(); // изменяем состояние singletone зависимости

    let dependency2 = root_context.get::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency2.read().await.str, "test2".to_string()); // видим измененное состояние в новом объекте

    //
}