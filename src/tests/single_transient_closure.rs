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
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_closure(|_| Ok(TransientDependency { str: "test".to_string() }), DependencyLifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[test]
fn single_transient_closure_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_closure_sync(|_| Ok(TransientDependency { str: "test".to_string() }), DependencyLifeCycle::Transient).unwrap();

    let dependency = root_context.resolve_sync::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[tokio::test]
async fn single_transient_async_closure() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_async_closure(
        |_: crate::DependencyContext| { async { Ok(TransientDependency { str: "test".to_string() }) }},
        DependencyLifeCycle::Transient
    ).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());
}

#[test]
fn single_transient_async_closure_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();

    root_context.register_async_closure_sync(
        |_: crate::DependencyContext| { async { Ok(TransientDependency { str: "test".to_string() }) }},
        DependencyLifeCycle::Transient
    ).unwrap();

    let dependency = root_context.resolve_sync::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());
}