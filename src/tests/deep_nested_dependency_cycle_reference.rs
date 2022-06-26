use crate::{
    Constructor,
    types::BuildDependencyResult,
};

#[allow(dead_code)]
struct TransientDependency1 {
    pub d2: TransientDependency2,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency1 {
    fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.resolve()?,
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency1 {
    async fn ctor(ctx: crate::DependencyContext) -> BuildDependencyResult<Self> {
        Ok(Self {
            d2: ctx.resolve().await?,
        })
    }
}

#[allow(dead_code)]
struct TransientDependency2 {
    pub d1: Box<TransientDependency3>,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency2 {
    fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.resolve()?),
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency2 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.resolve().await?),
        })
    }
}

#[allow(dead_code)]
struct TransientDependency3 {
    pub d1: Box<TransientDependency1>,
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency3 {
    fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.resolve()?),
        })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency3 {
    async fn ctor(ctx: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self {
            d1: Box::new(ctx.resolve().await?),
        })
    }
}

#[cfg(not(feature = "async-mode"))]
#[cfg(feature = "loop-check")]
#[test]
fn deep_nested_dependency_cycle_reference() {
    use crate::{DependencyContext, LifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.register_type::<TransientDependency3>(LifeCycle::Transient).unwrap();

    let dependency = root_context.resolve::<TransientDependency1>();

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        child_type_info: TypeInfo::from_type::<TransientDependency1>(),
        parent_type_info: TypeInfo::from_type::<TransientDependency3>(),
    }));
}

#[cfg(feature = "async-mode")]
#[cfg(feature = "loop-check")]
#[tokio::test]
async fn deep_nested_dependency_cycle_reference() {
    use crate::{DependencyContext, LifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency1>(LifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency2>(LifeCycle::Transient).await.unwrap();
    root_context.register_type::<TransientDependency3>(LifeCycle::Transient).await.unwrap();

    let dependency = root_context.resolve::<TransientDependency1>().await;

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        child_type_info: TypeInfo::from_type::<TransientDependency1>(),
        parent_type_info: TypeInfo::from_type::<TransientDependency3>(),
    }));
}

#[cfg(feature = "blocking")]
#[cfg(feature = "loop-check")]
#[test]
fn deep_nested_dependency_cycle_reference_sync() {
    use crate::{DependencyContext, LifeCycle};
    use crate::{
        types::{BuildDependencyError, TypeInfo},
    };
    
    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<TransientDependency1>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_type::<TransientDependency2>(LifeCycle::Transient).unwrap();
    root_context.blocking_register_type::<TransientDependency3>(LifeCycle::Transient).unwrap();

    let dependency = root_context.blocking_resolve::<TransientDependency1>();

    assert_eq!(dependency.err(), Some(BuildDependencyError::CyclicReference {
        child_type_info: TypeInfo::from_type::<TransientDependency1>(),
        parent_type_info: TypeInfo::from_type::<TransientDependency3>(),
    }));
}