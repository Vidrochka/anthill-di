use std::{
    any::TypeId,
    collections::HashMap,
    sync::Arc
};

use async_recursion::async_recursion;
use tokio::sync::RwLock;

#[derive(Debug)]
pub (crate) struct DependencyLink {
    pub parents: HashMap<TypeId, Arc<RwLock<DependencyLink>>>,
}

impl DependencyLink {
    pub (crate) fn new() -> Self {
        Self { parents: HashMap::new() }
    }

    pub (crate) fn with_parent(parent_id: TypeId, parent: Arc<RwLock<DependencyLink>>) -> Self {
        let mut link_collection = HashMap::new();
        link_collection.insert(parent_id, parent);
        Self { parents: link_collection }
    }

    pub (crate) fn add_parent(&mut self, parent_id: TypeId, parent: Arc<RwLock<DependencyLink>>) {
        self.parents.insert(parent_id, parent);
    }

    #[async_recursion(?Send)]
    pub (crate) async fn search_link(&self, id: &TypeId) -> bool {
        if let Some(_) = self.parents.get(&id) {
            return true;
        } else {
            for parent in self.parents.values() {
                if parent.read().await.search_link(&id).await {
                    return true;
                }
            }

            return false;
        }
    }
}