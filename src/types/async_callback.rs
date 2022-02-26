use std::pin::Pin;

pub type AsyncCallback<TIn, TOut> = Box<dyn Fn(TIn) -> Pin<Box<dyn std::future::Future<Output = TOut> + Send + 'static> > + Sync + Send>;
pub type AsyncBuilderCallback<TOut> = Box<dyn Fn() -> Pin<Box<dyn std::future::Future<Output = TOut> + Send + 'static> > + Sync + Send>;