use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
    pub dependency: TransientDependency2,
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
fn single_transient_closure() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.register_closure(|ctx| Ok(TransientDependency {
        str: "test".to_string(),
        dependency: ctx.resolve().unwrap(),
    }), LifeCycle::Transient).unwrap();

    let dependency = root_context.resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[tokio::test]
async fn single_transient_closure() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).await.unwrap();
    root_context.register_closure(|ctx| Ok(TransientDependency {
        str: "test".to_string(),
        dependency: ctx.blocking_resolve().unwrap(),
    }), LifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_transient_closure_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.blocking_register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_closure(|ctx| Ok(TransientDependency {
        str: "test".to_string(),
        dependency: ctx.blocking_resolve()?
    }), LifeCycle::Transient).unwrap();

    let dependency = root_context.blocking_resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_transient_async_closure() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).await.unwrap();
    root_context.register_async_closure(
        move |ctx: crate::DependencyContext| { async move {
            Ok(TransientDependency {
                str: "test".to_string(),
                dependency: ctx.resolve().await?,
            }
        ) }},
        LifeCycle::Transient
    ).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_transient_async_closure_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.blocking_register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_async_closure(
        |ctx: crate::DependencyContext| { async move { Ok(TransientDependency {
            str: "test".to_string(), 
            dependency: ctx.resolve().await?
        }) }},
        LifeCycle::Transient
    ).unwrap();

    let dependency = root_context.blocking_resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}