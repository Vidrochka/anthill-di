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

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for ContextDependentDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_context_dependent_interface() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).unwrap()
        .map_as::<AnthillRwLock<dyn GetStr>>().unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<dyn GetStr>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().unwrap().get(), "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_context_dependent_interface() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).await.unwrap()
        .map_as::<AnthillRwLock<dyn GetStr>>().await.unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<dyn GetStr>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.get(), "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_context_dependent_interface_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).unwrap()
        .blocking_map_as::<AnthillRwLock<dyn GetStr>>().unwrap();

    let dependency = root_context.blocking_resolve::<Weak<AnthillRwLock<dyn GetStr>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().get(), "test".to_string());
}