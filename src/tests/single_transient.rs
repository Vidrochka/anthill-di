use crate::{Constructor, types::BuildDependencyResult};

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

#[tokio::test]
async fn single_transient() {
    use crate::DependencyContext;
    use crate::DependencyLifeCycle;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(DependencyLifeCycle::Transient).await.unwrap();

    let mut dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());

    dependency.str = "test2".to_string(); // меняем состояние текущего объекта

    let dependency2 = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency2.str, "test".to_string()); // состояние нового объекта не синхронизировано с измененным объектом
}

#[test]
fn single_transient_sync() {
    use crate::DependencyContext;
    use crate::DependencyLifeCycle;

    let root_context = DependencyContext::new_root();
    root_context.register_type_sync::<TransientDependency>(DependencyLifeCycle::Transient).unwrap();

    let mut dependency = root_context.resolve_sync::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());

    dependency.str = "test2".to_string(); // меняем состояние текущего объекта

    let dependency2 = root_context.resolve_sync::<TransientDependency>().unwrap();

    assert_eq!(dependency2.str, "test".to_string()); // состояние нового объекта не синхронизировано с измененным объектом
}