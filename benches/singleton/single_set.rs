
use tokio::sync::RwLock;
use tokio::runtime::Runtime;
use async_trait::async_trait;
use anthill_di::{DependencyContext, types::BuildDependencyResult, Constructor, DependencyLifeCycle};

use criterion::{criterion_group, Criterion};

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

pub fn benchmark_single_singleton_set(c: &mut Criterion) {
    c.bench_function("benchmark_single_singleton_set", move |b| b.to_async(Runtime::new().unwrap()).iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context: DependencyContext| async {
        let root_context = root_context;
        root_context.register_type::<RwLock<SingletonDependency>>(DependencyLifeCycle::Singleton).await.unwrap();
    }));
}

criterion_group!(benches, benchmark_single_singleton_set);
