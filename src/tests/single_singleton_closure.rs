use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[async_trait(?Send)]
impl Constructor for SingletonDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_singleton_closure() {
    use crate::DependencyContext;
    use crate::extensions::ClosureDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_singleton_closure::<SingletonDependency>(
        Box::new(move |_: crate::DependencyContext| {
            Box::pin (async move {
                return Ok(SingletonDependency { str: "test".to_string() });
            })
        })
    ).await.unwrap();

    let dependency = root_context.get_singleton::<SingletonDependency>().await.unwrap();

    assert_eq!(dependency.read().await.str, "test".to_string());
}