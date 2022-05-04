use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
    pub dependency: TransientDependency2,
}

struct TransientDependency2 {
    pub str: String,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_transient_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_closure(|ctx| Ok(TransientDependency {
        str: "test".to_string(),
        dependency: ctx.resolve_sync().unwrap(),
    }), DependencyLifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[test]
fn single_transient_closure_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap();
    root_context.register_closure_sync(|ctx| Ok(TransientDependency {
        str: "test".to_string(),
        dependency: ctx.resolve_sync()?
    }), DependencyLifeCycle::Transient).unwrap();

    let dependency = root_context.resolve_sync::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[tokio::test]
async fn single_transient_async_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap();
    root_context.register_async_closure(
        move |ctx: crate::DependencyContext| { async move {
            Ok(TransientDependency {
                str: "test".to_string(),
                dependency: ctx.resolve().await?,
            }
        ) }},
        DependencyLifeCycle::Transient
    ).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[test]
fn single_transient_async_closure_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap();
    root_context.register_async_closure_sync(
        |ctx: crate::DependencyContext| { async move { Ok(TransientDependency {
            str: "test".to_string(), 
            dependency: ctx.resolve().await?
        }) }},
        DependencyLifeCycle::Transient
    ).unwrap();

    let dependency = root_context.resolve_sync::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}