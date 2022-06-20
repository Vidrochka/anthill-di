
use std::sync::Arc;

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
pub fn benchmark_singleton_get(c: &mut Criterion) {
    let root_context = DependencyContext::new_root();
    root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).unwrap();

    c.bench_function("benchmark_singleton_get", move |b| b.iter(
    || {
        let root_context = root_context.clone();
        black_box(root_context.resolve::<Arc<SingletonDependency>>().unwrap())
    }));
}

#[cfg(feature = "async-mode")]
pub fn benchmark_singleton_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let root_context = DependencyContext::new_root();

    rt.block_on(async { 
        root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).await.unwrap();
    });

    c.bench_function("benchmark_singleton_get_async", move |b| b.to_async(&rt).iter(
    || async {
        let root_context = root_context.clone();
        black_box(root_context.resolve::<Arc<SingletonDependency>>().await.unwrap())
    }));
}

criterion_group!(benches, benchmark_singleton_get);
