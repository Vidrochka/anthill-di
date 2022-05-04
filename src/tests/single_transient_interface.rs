use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
}

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

#[tokio::test]
async fn single_transient_interface() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();
    
    let dependency = root_context.resolve::<Box<dyn GetStr>>().await.unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}

#[test]
fn single_transient_interface_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<TransientDependency>(DependencyLifeCycle::Transient).unwrap()
        .map_as_sync::<dyn GetStr>().unwrap();
    
    let dependency = root_context.resolve_sync::<Box<dyn GetStr>>().unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}