use async_trait::async_trait;

use crate::{Constructor, types::BuildDependencyResult};

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

#[tokio::test]
async fn single_transient() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;

    let root_context = DependencyContext::new_root();
    root_context.set_transient::<TransientDependency>().await.unwrap();

    let mut dependency = root_context.get::<TransientDependency>().await.unwrap();

    assert_eq!(dependency.str, "test".to_string());

    dependency.str = "test2".to_string(); // меняем состояние текущего объекта

    let dependency2 = root_context.get::<TransientDependency>().await.unwrap();

    assert_eq!(dependency2.str, "test".to_string()); // состояние нового объекта не синхронизировано с измененным объектом
}