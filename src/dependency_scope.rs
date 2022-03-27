use std::{
    any::{
        Any,
        TypeId
    },
    collections::HashMap,
    sync::Arc
};

use tokio::sync::RwLock;

use crate::GlobalScope;

pub struct DependencyScope {
    pub (crate) local_scope: RwLock<HashMap<TypeId, Arc<RwLock<Option<Arc<dyn Any + Sync + Send>>>>>>,
    pub (crate) global_scope: Arc<RwLock<GlobalScope>>,
}

impl std::fmt::Debug for DependencyScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependencyScope")
            .field("local_scope", 
                &self.local_scope.try_read().unwrap()
                    .iter().map(|(id, instance)| (id.clone(), if instance.try_read().unwrap().is_none() {"None"} else {"Not Empty"}))
                    .collect::<HashMap<_,_>>()
            )
            .field("global_scope", &self.global_scope.try_read().unwrap()
        ).finish()
    }
}

impl DependencyScope {
    pub (crate) fn new(global_scope: Arc<RwLock<GlobalScope>>) -> Self { Self { local_scope: RwLock::new( HashMap::new() ), global_scope } }
}