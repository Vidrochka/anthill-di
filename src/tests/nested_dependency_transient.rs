use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d1: TransientDependency2,
    pub d2: TransientDependency2,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency1 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve()?,
            d2: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency2 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn nested_dependency_transient() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().unwrap();

    assert_eq!(dependency.d1.str, "test".to_string());
    assert_eq!(dependency.d2.str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn nested_dependency_transient() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await.unwrap();

    assert_eq!(dependency.d1.str, "test".to_string());
    assert_eq!(dependency.d2.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn nested_dependency_transient_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();

    let dependency = root_context.blocking_resolve::<TransientDependency1>().unwrap();

    assert_eq!(dependency.d1.str, "test".to_string());
    assert_eq!(dependency.d2.str, "test".to_string());
}