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
fn single_context_dependent() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::{Arc, Weak};

    let mut root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().unwrap().str, "test".to_string());

    let dependency2 = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert!(Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency2.upgrade().unwrap())); // ссылки на context dependent объекты созданные в одном local context совпадают

    let _old_context = root_context.get_context(); // сохраняем local context, т.к. при удалении ссылок на local context все context dependent зависимости удаленного local context удаяются
    let _new_context = root_context.set_empty_context(); // устанавливаем новый чистый local context

    let dependency3 = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert!(!Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency3.upgrade().unwrap())); // dependency и dependency3 ссылаются на разные объекты т.к. созданы в разных local context
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_context_dependent() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::{Arc, Weak};

    let mut root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).await.unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());

    let dependency2 = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().await.unwrap();

    assert!(Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency2.upgrade().unwrap())); // ссылки на context dependent объекты созданные в одном local context совпадают

    let _old_context = root_context.get_context(); // сохраняем local context, т.к. при удалении ссылок на local context все context dependent зависимости удаленного local context удаяются
    let _new_context = root_context.set_empty_context(); // устанавливаем новый чистый local context

    let dependency3 = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().await.unwrap();

    assert!(!Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency3.upgrade().unwrap())); // dependency и dependency3 ссылаются на разные объекты т.к. созданы в разных local context
}

#[cfg(feature = "blocking")]
#[test]
fn single_context_dependent_sync() {
    use crate::{
        types::AnthillRwLock,
        DependencyContext,
        LifeCycle
    };
    use std::sync::{Arc, Weak};

    let mut root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().str, "test".to_string());

    let dependency2 = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert!(Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency2.upgrade().unwrap())); // ссылки на context dependent объекты созданные в одном local context совпадают

    let _old_context = root_context.get_context(); // сохраняем local context, т.к. при удалении ссылок на local context все context dependent зависимости удаленного local context удаяются
    let _new_context = root_context.set_empty_context(); // устанавливаем новый чистый local context

    let dependency3 = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert!(!Arc::ptr_eq(&dependency.upgrade().unwrap(), &dependency3.upgrade().unwrap())); // dependency и dependency3 ссылаются на разные объекты т.к. созданы в разных local context
}