use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for SingletonDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_singleton_interface() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<SingletonDependency>>(LifeCycle::Singleton).unwrap()
        .map_as::<AnthillRwLock<dyn GetStr>>().unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<dyn GetStr>>>().unwrap();

    assert_eq!(dependency.read().unwrap().get(), "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_singleton_interface() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<SingletonDependency>>(LifeCycle::Singleton).await.unwrap()
        .map_as::<AnthillRwLock<dyn GetStr>>().await.unwrap();

    let dependency = root_context.resolve::<Arc<AnthillRwLock<dyn GetStr>>>().await.unwrap();

    assert_eq!(dependency.read().await.get(), "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_singleton_interface_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::Arc;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<SingletonDependency>>(LifeCycle::Singleton).unwrap()
        .blocking_map_as::<AnthillRwLock<dyn GetStr>>().unwrap();

    let dependency = root_context.blocking_resolve::<Arc<AnthillRwLock<dyn GetStr>>>().unwrap();

    assert_eq!(dependency.blocking_read().get(), "test".to_string());
}