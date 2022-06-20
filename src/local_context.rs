use std::{
    any::{
        Any,
        TypeId
    },
    collections::HashMap,
    sync::Arc
};

use crate::types::AnthillRwLock;

#[derive(Default)]
pub struct LocalContext {
    pub (crate) local_context: AnthillRwLock<HashMap<TypeId, Arc<AnthillRwLock<Option<Arc<dyn Any + Sync + Send>>>>>>,
}

impl std::fmt::Debug for LocalContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalContext")
            .field("local_context", 
                &self.local_context.try_read().unwrap()
                    .iter().map(|(id, instance)| (id.clone(), if instance.try_read().unwrap().is_none() {"None"} else {"Not Empty"}))
                    .collect::<HashMap<_,_>>()
            )
            .finish()
    }
}