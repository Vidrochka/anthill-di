use async_trait::async_trait;

use crate::{
    DependencyContext,
    types::BuildDependencyResult
};

#[async_trait]
pub trait Constructor where Self: Sized + 'static {
    async fn ctor(ctx: DependencyContext) -> BuildDependencyResult<Self>;
}