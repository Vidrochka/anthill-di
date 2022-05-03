use std::any::TypeId;

use async_trait::async_trait;

use crate::{Constructor, types::BuildDependencyResult};

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

#[allow(dead_code)]
struct TransientDependency1 {
    pub str: String,
}

#[async_trait]
impl Constructor for TransientDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test1".to_string() })
    }
}

impl GetStr for TransientDependency1 {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub str: String,
}

#[async_trait]
impl Constructor for TransientDependency2 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test2".to_string() })
    }
}

impl GetStr for TransientDependency2 {
    fn get(&self) -> String {
        self.str.clone()
    }
}

#[tokio::test]
async fn single_transient_resolve_by_type_id() {
    use crate::DependencyContext;
    use crate::DependencyLifeCycle;

    let root_context = DependencyContext::new_root();
    
    root_context.register_type::<TransientDependency1>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();
    root_context.register_type::<TransientDependency2>(DependencyLifeCycle::Transient).await.unwrap()
        .map_as::<dyn GetStr>().await.unwrap();

    let dependency1 = root_context.resolve_by_type_id::<Box<dyn GetStr>>(TypeId::of::<TransientDependency1>()).await.unwrap();
    let dependency2 = root_context.resolve_by_type_id::<Box<dyn GetStr>>(TypeId::of::<TransientDependency2>()).await.unwrap();

    assert_eq!(dependency1.get(), "test1".to_string());
    assert_eq!(dependency2.get(), "test2".to_string());
}

#[test]
fn single_transient_resolve_by_type_id_sync() {
    use crate::DependencyContext;
    use crate::DependencyLifeCycle;

    let root_context = DependencyContext::new_root();
    
    root_context.register_type_sync::<TransientDependency1>(DependencyLifeCycle::Transient).unwrap()
        .map_as_sync::<dyn GetStr>().unwrap();
    root_context.register_type_sync::<TransientDependency2>(DependencyLifeCycle::Transient).unwrap()
        .map_as_sync::<dyn GetStr>().unwrap();

    let dependency1 = root_context.resolve_by_type_id_sync::<Box<dyn GetStr>>(TypeId::of::<TransientDependency1>()).unwrap();
    let dependency2 = root_context.resolve_by_type_id_sync::<Box<dyn GetStr>>(TypeId::of::<TransientDependency2>()).unwrap();

    assert_eq!(dependency1.get(), "test1".to_string());
    assert_eq!(dependency2.get(), "test2".to_string());
}