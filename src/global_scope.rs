use tokio::sync::RwLock;
use std::{collections::HashMap, any::{TypeId, Any}, sync::Arc};


pub (crate) struct GlobalScope {
    pub (crate) singletons: HashMap<TypeId, Arc<RwLock<Option<Arc<dyn Any + Sync + Send>>>>>,
}

impl std::fmt::Debug for GlobalScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalScope")
            .field("singletons",
            &self.singletons.iter().map(|(id, instance)| (id, if instance.try_read().unwrap().is_none() {"None"} else {"Not Empty"})).collect::<HashMap<_, _>>())
            .finish()
    }
}

impl GlobalScope {
    #[must_use] pub (crate) fn new() -> Self { Self { singletons: HashMap::new() } }
}