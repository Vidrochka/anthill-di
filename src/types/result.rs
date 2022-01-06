use std::any::TypeId;

use crate::DependencyLifeCycle;

pub type BuildDependencyResult<T> = Result<T, BuildDependencyError>;

#[derive(Debug, PartialEq)]
pub enum BuildDependencyError {
    NotFound { id: TypeId, name: String, },
    ParentNotFound { id: TypeId, name: String, parent_id: TypeId, },
    InvalidCast { id: TypeId, name: String, },
    CyclicReference { id: TypeId, name: String, parent_id: TypeId, },
    InvalidLifeCycle { id: TypeId, name: String, expected: DependencyLifeCycle, requested: DependencyLifeCycle, },
    AddDependencyError { err: AddDependencyError },
    Custom { message: String, }
}

pub type AddDependencyResult<T> = Result<T, AddDependencyError>;

#[derive(Debug, PartialEq)]
pub enum AddDependencyError {
    DependencyExist { id: TypeId, name: String, }
}