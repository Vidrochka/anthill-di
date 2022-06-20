use std::sync::{Weak, Arc};


use crate::{
    Constructor,
    types::{
        BuildDependencyResult,
        AnthillRwLock,
    },
    LocalContext
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub s1: Weak<AnthillRwLock<ContextDependentDependency1>>,
    pub t1: TransientDependency2,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency1 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            s1: ctx.resolve()?,
            t1: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            s1: ctx.resolve().await?,
            t1: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub s2: Weak<AnthillRwLock<ContextDependentDependency1>>,
    pub t2: TransientDependency3,
    pub local_context: Arc<LocalContext>,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency2 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        let mut ctx = ctx;
        let local_context = ctx.set_empty_context();

        Ok(Self {
            s2: ctx.resolve()?,
            t2: ctx.resolve()?,
            local_context: local_context,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        let mut ctx = ctx;
        let local_context = ctx.set_empty_context();

        Ok(Self {
            s2: ctx.resolve().await?,
            t2: ctx.resolve().await?,
            local_context: local_context,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub s3: Weak<AnthillRwLock<ContextDependentDependency1>>,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency3 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            s3: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency3 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            s3: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct ContextDependentDependency1 {
    pub str: String,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for ContextDependentDependency1 {
    fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for ContextDependentDependency1 {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

#[cfg(not(feature = "async-mode"))]
#[test]
fn deep_nested_dependency_context_dependent() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency1>>(LifeCycle::ContextDependent).unwrap();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.register_type::<TransientDependency3>(LifeCycle::Transient).unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().unwrap();

    dependency.s1.upgrade().unwrap().write().unwrap().str = "test2".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().read().unwrap().str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().read().unwrap().str, "test".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().read().unwrap().str, "test".to_string());

    dependency.t1.s2.upgrade().unwrap().write().unwrap().str = "test3".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().read().unwrap().str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().read().unwrap().str, "test3".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().read().unwrap().str, "test3".to_string());
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn deep_nested_dependency_context_dependent() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency1>>(LifeCycle::ContextDependent).await.unwrap();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency3>(LifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await.unwrap();

    dependency.s1.upgrade().unwrap().write().await.str = "test2".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().read().await.str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().read().await.str, "test".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().read().await.str, "test".to_string());

    dependency.t1.s2.upgrade().unwrap().write().await.str = "test3".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().read().await.str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().read().await.str, "test3".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().read().await.str, "test3".to_string());
}

#[cfg(feature = "blocking")]
#[test]
fn deep_nested_dependency_context_dependent_sync() {
    use crate::{DependencyContext, LifeCycle};

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<ContextDependentDependency1>>(LifeCycle::ContextDependent).unwrap();
    root_context.blocking_register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_type::<TransientDependency3>(LifeCycle::Transient).unwrap();

    let dependency = root_context.blocking_resolve::<TransientDependency1>().unwrap();

    dependency.s1.upgrade().unwrap().blocking_write().str = "test2".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().blocking_read().str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().blocking_read().str, "test".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().blocking_read().str, "test".to_string());

    dependency.t1.s2.upgrade().unwrap().blocking_write().str = "test3".to_string();

    assert_eq!(dependency.s1.upgrade().unwrap().blocking_read().str, "test2".to_string());
    assert_eq!(dependency.t1.s2.upgrade().unwrap().blocking_read().str, "test3".to_string());
    assert_eq!(dependency.t1.t2.s3.upgrade().unwrap().blocking_read().str, "test3".to_string());
}