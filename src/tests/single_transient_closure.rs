use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for TransientDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_transient_closure() {
    use crate::DependencyContext;
    use crate::extensions::ClosureDependencySetStrategy;

    let root_context = DependencyContext::new_root();

    root_context.set_transient_closure::<TransientDependency>(
        Box::new(move |_: crate::DependencyContext| {
            Box::pin (async move {
                return Ok(TransientDependency { str: "test".to_string() });
            })
        })
    ).await.unwrap();

    let dependency = root_context.get_transient::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}