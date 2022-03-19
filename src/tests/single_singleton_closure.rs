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
    use crate::DependencyContext;
    use crate::extensions::ClosureDependencySetStrategy;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.set_singleton_closure::<RwLock<SingletonDependency>>(
        Box::new(move |_: crate::DependencyContext| {
            Box::pin (async move {
                return Ok(RwLock::new(SingletonDependency { str: "test".to_string() }));
            })
        })
    ).await.unwrap();

    let dependency = root_context.get::<Arc<RwLock<SingletonDependency>>>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());
}