use async_trait::async_trait;

use crate::{Constructor, types::BuildDependencyResult};

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

#[async_trait(?Send)]
impl Constructor for ScopedDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn single_scoped() {
    use crate::DependencyContext;
    use crate::extensions::ConstructedDependencySetStrategy;
    use std::sync::Arc;

    let mut root_context = DependencyContext::new_root();
    root_context.set_scoped::<ScopedDependency>().await.unwrap();

    let dependency = root_context.get_scoped::<ScopedDependency>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());

    let dependency2 = root_context.get_scoped::<ScopedDependency>().await.unwrap();

    assert!(Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency2.upgrade().unwrap())); // ссылки на scoped объекты созданные в одном scope совпадают

    let _old_scope = root_context.get_scope(); // сохраняем scope, т.к. при удалении ссылок на scope все scoped зависимости удаленного scope удаяются
    let _new_scope = root_context.set_empty_scope(); // устанавливаем новый чистый scope

    let dependency3 = root_context.get_scoped::<ScopedDependency>().await.unwrap();

    assert!(!Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency3.upgrade().unwrap())); // dependency и dependency3 ссылаются на разные объекты т.к. созданы в разных scope
}