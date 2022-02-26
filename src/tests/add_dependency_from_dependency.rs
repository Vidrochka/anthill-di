use async_trait::async_trait;

use crate::{
    extensions::ConstructedDependencySetStrategy,
    Constructor,
    types::{
        BuildDependencyResult,
        BuildDependencyError
    }
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d1: TransientDependency2,
}

#[async_trait]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        // добавляем зависимость из собранной зависимости
        ctx.set_transient::<TransientDependency3>().await.map_err(|e| BuildDependencyError::AddDependencyError { err: e })?;

        Ok(Self {
            d1: ctx.get().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub d2: TransientDependency3,
}

#[async_trait]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.get().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub str: String,
}

#[async_trait]
impl Constructor for TransientDependency3 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_transient() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_transient::<TransientDependency1>().await.unwrap();
    root_context.set_transient::<TransientDependency2>().await.unwrap();

    let dependency = root_context.get::<TransientDependency1>().await.unwrap();

    assert_eq!(dependency.d1.d2.str, "test".to_string());
}