use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub str: String,
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub str: String,
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub str: String,
}

#[async_trait]
impl Constructor for TransientDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test1".to_string() })
    }
}

#[async_trait]
impl Constructor for TransientDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test2".to_string() })
    }
}

#[async_trait]
impl Constructor for TransientDependency3 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test3".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for TransientDependency1 {
    fn get(&self) -> String {
        self.str.clone()
    }
}

impl GetStr for TransientDependency2 {
    fn get(&self) -> String {
        self.str.clone()
    }
}

impl GetStr for TransientDependency3 {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[tokio::test]
async fn multiple_service_realization() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();

    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();

    root_context.register_type::<TransientDependency3>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();
    
    let dependency = root_context.resolve_collection::<Box<dyn GetStr>>().await.unwrap();
    let mut text_collection = dependency.iter().map(|x| x.get()).collect::<Vec<_>>();
    text_collection.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(text_collection, vec!["test1", "test2", "test3"]);
}

#[test]
fn multiple_service_realization_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<TransientDependency1>(DependencyLifeCycle::Transient).unwrap()
        .map_as_sync::<dyn GetStr>().unwrap();

    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap()
        .map_as_sync::<dyn GetStr>().unwrap();

    root_context.register_type_sync::<TransientDependency3>(DependencyLifeCycle::Transient).unwrap()
        .map_as_sync::<dyn GetStr>().unwrap();
    
    let dependency = root_context.resolve_collection_sync::<Box<dyn GetStr>>().unwrap();
    let mut text_collection = dependency.iter().map(|x| x.get()).collect::<Vec<_>>();
    text_collection.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(text_collection, vec!["test1", "test2", "test3"]);
}