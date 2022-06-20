#[cfg(feature = "async-mode")]
use tokio::runtime::Runtime;
use anthill_di::{DependencyContext, types::BuildDependencyResult, Constructor, LifeCycle};

use criterion::{criterion_group, Criterion};

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
pub fn benchmark_singleton_set(c: &mut Criterion) {
    c.bench_function("benchmark_singleton_set", move |b| b.iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context: DependencyContext| {
        let root_context = root_context;
        root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).unwrap();
    }));
}

#[cfg(feature = "async-mode")]
pub fn benchmark_singleton_set(c: &mut Criterion) {
    c.bench_function("benchmark_singleton_set_async", move |b| b.to_async(Runtime::new().unwrap()).iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context: DependencyContext| async {
        let root_context = root_context;
        root_context.register_type::<SingletonDependency>(LifeCycle::Singleton).await.unwrap();
    }));
}

criterion_group!(benches, benchmark_singleton_set);
