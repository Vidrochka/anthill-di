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
        types::{
            AnthillRwLock,
            DeleteComponentError,
            TypeInfo
        },
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().unwrap().str, "test".to_string());

    let result = root_context.delete_component::<AnthillRwLock<ContextDependentDependency>>();

    assert_eq!(result, Err(DeleteComponentError::NotSupportedLifeCycle {
        component_type_info: TypeInfo::from_type::<AnthillRwLock<ContextDependentDependency>>(),
        life_cycle: LifeCycle::ContextDependent
    }))
}

#[cfg(feature = "async-mode")]
#[tokio::test]
async fn single_context_dependent() {
    use crate::{
        types::{
            AnthillRwLock,
            DeleteComponentError,
            TypeInfo
        },
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;
    let root_context = DependencyContext::new_root();
    root_context.register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).await.unwrap();

    let dependency = root_context.resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().await.unwrap();

    assert_eq!(dependency.upgrade().unwrap().read().await.str, "test".to_string());

    let result = root_context.delete_component::<AnthillRwLock<ContextDependentDependency>>().await;

    assert_eq!(result, Err(DeleteComponentError::NotSupportedLifeCycle {
        component_type_info: TypeInfo::from_type::<AnthillRwLock<ContextDependentDependency>>(),
        life_cycle: LifeCycle::ContextDependent
    }))
}

#[cfg(feature = "blocking")]
#[test]
fn single_context_dependent_sync() {
    use crate::{
        types::{
            AnthillRwLock,
            DeleteComponentError,
            TypeInfo
        },
        DependencyContext,
        LifeCycle
    };
    use std::sync::Weak;

    let root_context = DependencyContext::new_root();
    root_context.blocking_register_type::<AnthillRwLock<ContextDependentDependency>>(LifeCycle::ContextDependent).unwrap();

    let dependency = root_context.blocking_resolve::<Weak<AnthillRwLock<ContextDependentDependency>>>().unwrap();

    assert_eq!(dependency.upgrade().unwrap().blocking_read().str, "test".to_string());

    let result = root_context.blocking_delete_component::<AnthillRwLock<ContextDependentDependency>>();

    assert_eq!(result, Err(DeleteComponentError::NotSupportedLifeCycle {
        component_type_info: TypeInfo::from_type::<AnthillRwLock<ContextDependentDependency>>(),
        life_cycle: LifeCycle::ContextDependent
    }))
}