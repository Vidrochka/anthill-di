use crate::{Constructor, types::BuildDependencyResult};

#[allow(dead_code)]
struct ContextDependentDependency {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn remove_context_dependent_dependency() {
    use crate::{DependencyContext, LifeCycle};
    use std::sync::Weak;

    let mut root_context = DependencyContext::new_root();
    root_context.register_type::<ContextDependentDependency>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.resolve::<Weak<ContextDependentDependency>>().unwrap();

    assert!(dependency.upgrade().is_some());

    let _new_context = root_context.set_empty_context(); // устанавливаем новый чистый local context, ссылка на старый скоуп ни где не сохранена 

    assert!(dependency.upgrade().is_none()); // зависимость удалилась, т.к. не осталось ссылок на скоуп

    let dependency = root_context.resolve::<Weak<ContextDependentDependency>>().unwrap();

    root_context.set_empty_context(); // устанавливаем новый чистый local context, ссылка на старый скоуп ранее сохранена в new_context

    assert!(dependency.upgrade().is_some()); // зависимость не удалилась, т.к. осталась ссылока на скоуп
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn remove_context_dependent_dependency() {
    use crate::{DependencyContext, LifeCycle};
    use std::sync::Weak;

    let mut root_context = DependencyContext::new_root();
    root_context.register_type::<ContextDependentDependency>(LifeCycle::ContextDependent).await.unwrap();

    let dependency = root_context.resolve::<Weak<ContextDependentDependency>>().await.unwrap();

    assert!(dependency.upgrade().is_some());

    let _new_context = root_context.set_empty_context(); // устанавливаем новый чистый local context, ссылка на старый скоуп ни где не сохранена 

    assert!(dependency.upgrade().is_none()); // зависимость удалилась, т.к. не осталось ссылок на скоуп

    let dependency = root_context.resolve::<Weak<ContextDependentDependency>>().await.unwrap();

    root_context.set_empty_context(); // устанавливаем новый чистый local context, ссылка на старый скоуп ранее сохранена в new_context

    assert!(dependency.upgrade().is_some()); // зависимость не удалилась, т.к. осталась ссылока на скоуп
}

#[cfg(feature = "blocking")]
#[test]
fn remove_context_dependent_dependency_sync() {
    use crate::{DependencyContext, LifeCycle};
    use std::sync::Weak;

    let mut root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<ContextDependentDependency>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.blocking_resolve::<Weak<ContextDependentDependency>>().unwrap();

    assert!(dependency.upgrade().is_some());

    let _new_context = root_context.set_empty_context(); // устанавливаем новый чистый local context, ссылка на старый скоуп ни где не сохранена 

    assert!(dependency.upgrade().is_none()); // зависимость удалилась, т.к. не осталось ссылок на скоуп

    let dependency = root_context.blocking_resolve::<Weak<ContextDependentDependency>>().unwrap();

    root_context.set_empty_context(); // устанавливаем новый чистый local context, ссылка на старый скоуп ранее сохранена в new_context

    assert!(dependency.upgrade().is_some()); // зависимость не удалилась, т.к. осталась ссылока на скоуп
}