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

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for SingletonDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[tokio::test]
async fn single_singleton_interface() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<RwLock<SingletonDependency>>(DependencyLifeCycle::Singleton).await.unwrap()
        .map_as::<RwLock<dyn GetStr>>().await.unwrap();

    let dependency = root_context.resolve::<Arc<RwLock<dyn GetStr>>>().await.unwrap();

    assert_eq!(dependency.read().await.get(), "test".to_string());
}

#[test]
fn single_singleton_interface_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<RwLock<SingletonDependency>>(DependencyLifeCycle::Singleton).unwrap()
        .map_as_sync::<RwLock<dyn GetStr>>().unwrap();

    let dependency = root_context.resolve_sync::<Arc<RwLock<dyn GetStr>>>().unwrap();

    assert_eq!(dependency.blocking_read().get(), "test".to_string());
}