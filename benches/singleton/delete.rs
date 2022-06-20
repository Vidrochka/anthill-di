
use std::time::Instant;

#[cfg(feature = "async-mode")]
use tokio::runtime::Runtime;
use anthill_di::{DependencyContext, types::BuildDependencyResult, Constructor, LifeCycle};

use criterion::{black_box, criterion_group, Criterion};

#[allow(dead_code)]
struct SingletonDependency {
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for SingletonDependency {
    fn ctor(_: anthill_di::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for SingletonDependency {
    async fn ctor(_: anthill_di::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { })
    }
}

#[cfg(not(feature = "async-mode"))]
pub fn benchmark_singleton_delete(c: &mut Criterion) {
    c.bench_function("benchmark_singleton_delete", move |b| b.iter_custom(|iters| {
        let mut root_contexts = Vec::new();

        for _ in 0..iters {
            let root_context = DependencyContext::new_root();
            root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).unwrap();
            root_contexts.push(root_context);
        }

        let start = Instant::now();

        for i in 0..iters {
            black_box(root_contexts[i as usize].delete_component::<SingletonDependency>().unwrap())
        }

        start.elapsed()
    }));
}

#[cfg(feature = "async-mode")]
pub fn benchmark_singleton_delete(c: &mut Criterion) {
    c.bench_function("benchmark_singleton_delete_async", move |b| b.to_async(Runtime::new().unwrap()).iter_custom(|iters| async move {
        let mut root_contexts = Vec::new();

        for _ in 0..iters {
            let root_context = DependencyContext::new_root();
            root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).await.unwrap();
            root_contexts.push(root_context);
        }

        let start = Instant::now();

        for i in 0..iters {
            black_box(root_contexts[i as usize].delete_component::<SingletonDependency>().await.unwrap())
        }

        start.elapsed()
    }));
}

criterion_group!(benches, benchmark_singleton_delete);
