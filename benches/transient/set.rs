#[cfg(feature = "async-mode")]
use tokio::runtime::Runtime;
use anthill_di::{DependencyContext, types::BuildDependencyResult, Constructor, LifeCycle};

use criterion::{criterion_group, Criterion};

#[allow(dead_code)]
struct TransientDependency {
}

#[cfg(not(feature = "async-mode"))]
impl Constructor for TransientDependency {
    fn ctor(_: anthill_di::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { })
    }
}

#[cfg(feature = "async-mode")]
#[async_trait_with_sync::async_trait(Sync)]
impl Constructor for TransientDependency {
    async fn ctor(_: anthill_di::DependencyContext) ->  BuildDependencyResult<Self> {
        Ok(Self { })
    }
}

#[cfg(not(feature = "async-mode"))]
pub fn benchmark_transient_set(c: &mut Criterion) {
    c.bench_function("benchmark_transient_set", move |b| b.iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context| {
        let root_context = root_context;
        root_context.register_type::<TransientDependency>(LifeCycle::Transient).unwrap();
    }));
}

#[cfg(feature = "async-mode")]
pub fn benchmark_transient_set(c: &mut Criterion) {
    c.bench_function("benchmark_transient_set_async", move |b| b.to_async(Runtime::new().unwrap()).iter_with_setup(|| {
        DependencyContext::new_root()
    },
    |root_context| async {
        let root_context = root_context;
        root_context.register_type::<TransientDependency>(LifeCycle::Transient).await.unwrap();
    }));
}

criterion_group!(benches, benchmark_transient_set);