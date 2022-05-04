use crate::{Constructor, types::BuildDependencyResult};

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ScopedDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[tokio::test]
async fn remove_scoped_dependency() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Weak;

    let mut root_context = DependencyContext::new_root();
    root_context.register_type::<ScopedDependency>(DependencyLifeCycle::Scoped).await.unwrap();

    let dependency = root_context.resolve::<Weak<ScopedDependency>>().await.unwrap();

    assert!(dependency.upgrade().is_some());

    let _new_scope = root_context.set_empty_scope(); // устанавливаем новый чистый scope, ссылка на старый скоуп ни где не сохранена 

    assert!(dependency.upgrade().is_none()); // зависимость удалилась, т.к. не осталось ссылок на скоуп

    let dependency = root_context.resolve::<Weak<ScopedDependency>>().await.unwrap();

    root_context.set_empty_scope(); // устанавливаем новый чистый scope, ссылка на старый скоуп ранее сохранена в new_scope

    assert!(dependency.upgrade().is_some()); // зависимость не удалилась, т.к. осталась ссылока на скоуп
}

#[test]
fn remove_scoped_dependency_sync() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Weak;

    let mut root_context = DependencyContext::new_root();
    root_context.register_type_sync::<ScopedDependency>(DependencyLifeCycle::Scoped).unwrap();

    let dependency = root_context.resolve_sync::<Weak<ScopedDependency>>().unwrap();

    assert!(dependency.upgrade().is_some());

    let _new_scope = root_context.set_empty_scope(); // устанавливаем новый чистый scope, ссылка на старый скоуп ни где не сохранена 

    assert!(dependency.upgrade().is_none()); // зависимость удалилась, т.к. не осталось ссылок на скоуп

    let dependency = root_context.resolve_sync::<Weak<ScopedDependency>>().unwrap();

    root_context.set_empty_scope(); // устанавливаем новый чистый scope, ссылка на старый скоуп ранее сохранена в new_scope

    assert!(dependency.upgrade().is_some()); // зависимость не удалилась, т.к. осталась ссылока на скоуп
}