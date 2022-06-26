use std::sync::Weak;

use crate::{
    Constructor,
    types::{
        BuildDependencyResult,
        AnthillRwLock,
    }
};

#[allow(dead_code)]
struct ContextDependentDependency1 {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency1 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[allow(dead_code)]
struct ContextDependentDependency2 {
    pub d1: Weak<AnthillRwLock<ContextDependentDependency1>>,
    pub d2: Weak<AnthillRwLock<ContextDependentDependency1>>,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency2 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve()?,
            d2: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d1: ctx.resolve().await?,
            d2: ctx.resolve().await?,
        })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn nested_dependency_context_dependent() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency1>>(LifeCycle::ContextDependent).unwrap();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency2>>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency2>>>().unwrap();

    dependency.upgrade().unwrap().read().unwrap().d1.upgrade().unwrap().write().unwrap().str = "test2".to_string();

    assert_eq!(dependency.upgrade().unwrap().read().unwrap().d2.upgrade().unwrap().read().unwrap().str, "test2".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn nested_dependency_context_dependent() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency1>>(LifeCycle::ContextDependent).await.unwrap();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency2>>(LifeCycle::ContextDependent).await.unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency2>>>().await.unwrap();

    dependency.upgrade().unwrap().read().await.d1.upgrade().unwrap().write().await.str = "test2".to_string();

    assert_eq!(dependency.upgrade().unwrap().read().await.d2.upgrade().unwrap().read().await.str, "test2".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn nested_dependency_context_dependent_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<ContextDependentDependency1>>(LifeCycle::ContextDependent).unwrap();
    root_context.blocking_register_type::<AnthillRwLock<ContextDependentDependency2>>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency2>>>().unwrap();

    dependency.upgrade().unwrap().blocking_read().d1.upgrade().unwrap().blocking_write().str = "test2".to_string();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().d2.upgrade().unwrap().blocking_read().str, "test2".to_string());
}