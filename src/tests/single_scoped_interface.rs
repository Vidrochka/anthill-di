use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for ScopedDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for ScopedDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[tokio::test]
async fn single_scoped_interface() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Weak;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<RwLock<ScopedDependency>>(DependencyLifeCycle::Scoped).await.unwrap()
        .map_as::<RwLock<dyn GetStr>>().await.unwrap();

    let dependency = root_context.resolve::<Weak<RwLock<dyn GetStr>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.get(), "test".to_string());
}