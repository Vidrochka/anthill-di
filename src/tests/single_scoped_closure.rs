use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for ScopedDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_scoped_closure() {
    use crate::DependencyContext;
    use crate::extensions::ClosureDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_scoped_closure::<RwLock<ScopedDependency>>(
        Box::new(move |_: crate::DependencyContext| {
            Box::pin (async move {
                return Ok(RwLock::new(ScopedDependency { str: "test".to_string() }));
            })
        })
    ).await.unwrap();

    let dependency = root_context.get_scoped::<RwLock<ScopedDependency>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());
}