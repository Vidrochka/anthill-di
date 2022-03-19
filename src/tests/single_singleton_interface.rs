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
    use crate::DependencyContext;
    use crate::extensions::InterfaceDependencySetStrategy;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.set_singleton_interface::<RwLock<dyn GetStr>, RwLock<SingletonDependency>>().await.unwrap();

    let dependency = root_context.get::<Arc<Box<RwLock<dyn GetStr>>>>().await.unwrap();

    assert_eq!(dependency.read().await.get(), "test".to_string());
}