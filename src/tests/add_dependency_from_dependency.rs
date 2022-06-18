use crate::{
    Constructor,
    DependencyLifeCycle,
    types::{
        BuildDependencyResult,
        BuildDependencyError
    }
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d1: TransientDependency2,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        // добавляем зависимость из собранной зависимости
        ctx.register_type::<TransientDependency3>(DependencyLifeCycle::Transient).await.map_err(|e| BuildDependencyError::AddDependencyError { err: e })?;

        Ok(Self {
            d1: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub d2: TransientDependency3,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub str: String,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency3 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn add_dependency_from_dependency() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await.unwrap();

    assert_eq!(dependency.d1.d2.str, "test".to_string());
}

#[test]
fn add_dependency_from_dependency_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<TransientDependency1>(DependencyLifeCycle::Transient).unwrap();
    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap();
    
    let dependency = root_context.resolve_sync::<TransientDependency1>().unwrap();

    assert_eq!(dependency.d1.d2.str, "test".to_string());
}