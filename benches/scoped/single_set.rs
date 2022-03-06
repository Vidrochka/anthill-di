
use tokio::sync::RwLock;
use tokio::runtime::Runtime;
use async_trait::async_trait;
use anthill_di::{DependencyContext, extensions::ConstructedDependencySetStrategy, types::BuildDependencyResult, Constructor};

use criterion::{criterion_group, Criterion};

#[allow(dead_code)]
struct ScopedDependency {
    pub str: String,
}

#[async_trait]
impl Constructor for ScopedDependency {
    async fn ctor(_: anthill_di::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { str: "test".to_string() })
    }
}

pub fn benchmark_single_scoped_set(c: &mut Criterion) {
    c.bench_function("benchmark_single_scoped_set", move |b| b.to_async(Runtime::new().unwrap()).iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context| async {
        let root_context = root_context;
        root_context.set_scoped::<RwLock<ScopedDependency>>().await.unwrap();
    }));
}

criterion_group!(benches, benchmark_single_scoped_set);