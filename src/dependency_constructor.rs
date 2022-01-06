use async_trait::async_trait;

use crate::{
    DependencyContext,
    types::BuildDependencyResult
};

#[async_trait(?Send)]
pub trait Constructor where Self: Sized + Send + 'static {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self>;
}