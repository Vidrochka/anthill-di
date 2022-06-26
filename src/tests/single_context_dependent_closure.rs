use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct ContextDependentDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_context_dependent_closure() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.register_closure(|_| Ok(AnthillRwLock::new(ContextDependentDependency { str: "test".to_string() })), LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().unwrap().str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_context_dependent_closure() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.register_closure(|_| Ok(AnthillRwLock::new(ContextDependentDependency { str: "test".to_string() })), LifeCycle::ContextDependent).await.unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_context_dependent_closure_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_closure(|_| Ok(AnthillRwLock::new(ContextDependentDependency { str: "test".to_string() })), LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().str, "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_context_dependent_async_closure() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.register_async_closure(
        move |_: crate::DependencyContext| {async move { Ok(AnthillRwLock::new(ContextDependentDependency { str: "test".to_string() })) }},
        LifeCycle::ContextDependent
    ).await.unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_context_dependent_async_closure_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_async_closure(
        move |_: crate::DependencyContext| {async move { Ok(AnthillRwLock::new(ContextDependentDependency { str: "test".to_string() })) }},
        LifeCycle::ContextDependent
    ).unwrap();

    let dependency = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().str, "test".to_string());
}