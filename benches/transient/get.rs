#[cfg(feature = "async-mode")]
use tokio::runtime::Runtime;
use anthill_di::{DependencyContext, types::BuildDependencyResult, Constructor, LifeCycle};

use criterion::{black_box, criterion_group, Criterion};

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
pub fn benchmark_transient_get(c: &mut Criterion) {
    let root_context = DependencyContext::new_root();
    root_context.register_type::<TransientDependency>(LifeCycle::Transient).unwrap();

    c.bench_function("benchmark_transient_get", move |b| b.iter(
    || {
        let root_context = root_context.clone();
        black_box(root_context.resolve::<TransientDependency>().unwrap());
    }));
}

#[cfg(feature = "async-mode")]
pub fn benchmark_transient_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let root_context = DependencyContext::new_root();

    rt.block_on(async { 
        root_context.register_type::<TransientDependency>(LifeCycle::Transient).await.unwrap();
    });

    c.bench_function("benchmark_transient_get_async", move |b| b.to_async(&rt).iter(
    || async {
        let root_context = root_context.clone();
        black_box(root_context.resolve::<TransientDependency>().await.unwrap());
    }));
}

criterion_group!(benches, benchmark_transient_get);