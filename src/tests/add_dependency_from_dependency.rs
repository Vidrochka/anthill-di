use crate::{
    Constructor,
    LifeCycle,
    types::{
        BuildDependencyResult,
        BuildDependencyError
    }
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d1: TransientDependency2,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency1 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        // добавляем зависимость из собранной зависимости
        ctx.register_type::<TransientDependency3>(LifeCycle::Transient).map_err(|e| BuildDependencyError::AddDependencyError { err: e })?;

        Ok(Self {
            d1: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        // добавляем зависимость из собранной зависимости
        ctx.register_type::<TransientDependency3>(LifeCycle::Transient).await.map_err(|e| BuildDependencyError::AddDependencyError { err: e })?;

        Ok(Self {
            d1: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub d2: TransientDependency3,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency2 {
    fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
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

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency3 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency3 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn add_dependency_from_dependency() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().unwrap();

    assert_eq!(dependency.d1.d2.str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn add_dependency_from_dependency() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await.unwrap();

    assert_eq!(dependency.d1.d2.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn add_dependency_from_dependency_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    
    let dependency = root_context.blocking_resolve::<TransientDependency1>().unwrap();

    assert_eq!(dependency.d1.d2.str, "test".to_string());
}