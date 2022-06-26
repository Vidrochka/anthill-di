use std::{collections::HashMap, any::{TypeId, Any}, sync::Arc};
use derive_new::new;

use crate::types::AnthillRwLock;

#[derive(Default, new)]
pub (crate) struct GlobalContext {
    #[new(default)]
    pub (crate) singletons: HashMap<TypeId, Arc<AnthillRwLock<Option<Arc<dyn Any + Sync + Send>>>>>,
}

impl std::fmt::Debug for GlobalContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalContext")
            .field("singletons",
            &self.singletons.iter().map(|(id, instance)| (id, if instance.try_read().unwrap().is_none() {"None"} else {"Not Empty"})).collect::<HashMap<_, _>>())
            .finish()
    }
}