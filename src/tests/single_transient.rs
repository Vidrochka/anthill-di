use crate::{Constructor, types::BuildDependencyResult};

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

#[cfg(not(feature = "async-mode"))]
#[test]
fn single_transient() {
    use crate::DependencyContext;
    use crate::LifeCycle;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).unwrap();

    let mut dependency = root_context.resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());

    dependency.str = "test2".to_string(); // меняем состояние текущего объекта

    let dependency2 = root_context.resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency2.str, "test".to_string()); // состояние нового объекта не синхронизировано с измененным объектом
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_transient() {
    use crate::DependencyContext;
    use crate::LifeCycle;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).await.unwrap();

    let mut dependency = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());

    dependency.str = "test2".to_string(); // меняем состояние текущего объекта

    let dependency2 = root_context.resolve::<TransientDependency>().await.unwrap();

    assert_eq!(dependency2.str, "test".to_string()); // состояние нового объекта не синхронизировано с измененным объектом
}

#[cfg(feature = "blocking")]
#[test]
fn single_transient_sync() {
    use crate::DependencyContext;
    use crate::LifeCycle;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency>(LifeCycle::Transient).unwrap();

    let mut dependency = root_context.blocking_resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency.str, "test".to_string());

    dependency.str = "test2".to_string(); // меняем состояние текущего объекта

    let dependency2 = root_context.blocking_resolve::<TransientDependency>().unwrap();

    assert_eq!(dependency2.str, "test".to_string()); // состояние нового объекта не синхронизировано с измененным объектом
}