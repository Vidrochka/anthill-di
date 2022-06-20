use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for TransientDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_transient_interface() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).unwrap()
        .map_as::<dyn GetStr>().unwrap();
    
    let dependency = root_context.resolve::<Box<dyn GetStr>>().unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_transient_interface() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();
    
    let dependency = root_context.resolve::<Box<dyn GetStr>>().await.unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn single_transient_interface_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency>(LifeCycle::Transient).unwrap()
        .blocking_map_as::<dyn GetStr>().unwrap();
    
    let dependency = root_context.blocking_resolve::<Box<dyn GetStr>>().unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}