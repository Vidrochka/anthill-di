use async_trait::async_trait;

use crate::{
    Constructor,
    types::BuildDependencyResult
};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for SingletonDependency {
    async fn ctor(_: crate::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

trait GetStr: Sync + Send {
    fn get(&self) -> String;
}

impl GetStr for SingletonDependency {
    fn get(&self) -> String {
        self.str.clone()
    }
}

// impl std::ops::CoerceUnsized<tokio::sync::RwLock<U>> for tokio::sync::RwLock<T>
// where
//     T: std::marker::Unsize<U> + ?Sized,
//     U: ?Sized,
// {

// }

#[tokio::test]
async fn single_singleton_interface() {
    use crate::{DependencyContext, DependencyLifeCycle};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    let root_context = DependencyContext::new_root();
    root_context.register::<RwLock<SingletonDependency>>(DependencyLifeCycle::Singleton).await.unwrap();
    println!("{root_context:#?}");
    //root_context.map_component_as_trait_service::<Arc<RwLock<SingletonDependency>>, Arc<RwLock<dyn GetStr>>>().await.unwrap();
    //root_context.set_singleton_interface::<RwLock<dyn GetStr>, RwLock<SingletonDependency>>().await.unwrap();

    let t = Arc::new(RwLock::new(SingletonDependency{ str: "test".to_string()}));
    //let t2: Arc<RwLock<dyn GetStr>> = t.;

    println!("{root_context:#?}");

    let dependency = root_context.get::<Arc<RwLock<dyn GetStr>>>().await.unwrap();

    assert_eq!(dependency.read().await.get(), "test".to_string());
}