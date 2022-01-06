use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d1: TransientDependency2,
    pub d2: TransientDependency2,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.get_transient().await?,
            d2: ctx.get_transient().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub str: String,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn nested_dependency_transient() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_transient::<TransientDependency1>().await.unwrap();
    root_context.set_transient::<TransientDependency2>().await.unwrap();

    let dependency = root_context.get_transient::<TransientDependency1>().await.unwrap();

    assert_eq!(dependency.d1.str, "test".to_string());
    assert_eq!(dependency.d2.str, "test".to_string());
}