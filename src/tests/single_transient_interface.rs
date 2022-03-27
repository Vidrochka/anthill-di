use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
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
    root_context.register::<TransientDependency>(DependencyLifeCycle::Transient).await.unwrap()
        .map_component_as_trait_service::<TransientDependency, dyn GetStr>().await.unwrap();
    
    println!("{root_context:#?}");

    let dependency = root_context.get::<Box<dyn GetStr>>().await.unwrap();

    assert_eq!(dependency.get(), "test".to_string());

    println!("{root_context:#?}");
}