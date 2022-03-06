
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::runtime::Runtime;
use async_trait::async_trait;
use anthill_di::{DependencyContext, extensions::ConstructedDependencySetStrategy, types::BuildDependencyResult, Constructor};

use criterion::{black_box, criterion_group, Criterion};

#[allow(dead_code)]
struct SingletonDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for SingletonDependency {
    async fn ctor(_: anthill_di::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

pub fn benchmark_single_singleton_set_get(c: &mut Criterion) {
    c.bench_function("benchmark_single_singleton_set_get", move |b| b.to_async(Runtime::new().unwrap()).iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context: DependencyContext| async {
        let root_context = root_context;
        root_context.set_singleton::<RwLock<SingletonDependency>>().await.unwrap();
        black_box(root_context.get::<Arc<RwLock<SingletonDependency>>>().await.unwrap())
    }));
}

criterion_group!(benches, benchmark_single_singleton_set_get);
