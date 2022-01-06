use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct TransientDependency {
    pub str: String,
}

#[async_trait(?Send)]
impl Constructor for TransientDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr {
    fn get(&self) -> String;
}

impl GetStr for TransientDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[tokio::test]
async fn single_transient_interface() {
    use crate::DependencyContext;
    use crate::extensions::InterfaceDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_transient_interface::<dyn GetStr, TransientDependency>().await.unwrap();

    let dependency = root_context.get_transient::<Box<dyn GetStr>>().await.unwrap();

    assert_eq!(dependency.get(), "test".to_string());
}