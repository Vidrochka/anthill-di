use super::TypeInfo;

pub type BuildDependencyResult<T> = Result<T, BuildDependencyError>;

#[derive(Debug, PartialEq)]
pub enum BuildDependencyError {
    NotFound { type_info: TypeInfo },
    //ParentNotFound { child_info: TypeInfo, parent_id: TypeId, },
    //InvalidCast { from_id: TypeId, from_name: String, to_id: TypeId, to_name: String },
    CyclicReference { child_type_info: TypeInfo, parent_type_info: TypeInfo },
    //InvalidLifeCycle { id: TypeId, name: String, expected: DependencyLifeCycle, requested: DependencyLifeCycle, },
    AddDependencyError { err: AddDependencyError },
    Custom { message: String, }
}

pub type AddDependencyResult<T> = Result<T, AddDependencyError>;

#[derive(Debug, PartialEq)]
pub enum AddDependencyError {
    DependencyExist { type_info: TypeInfo, },   
}

pub type MapComponentResult<T> = Result<T, MapComponentError>;

#[derive(Debug, PartialEq)]
pub enum MapComponentError {
    ComponentNotFound { type_info: TypeInfo },
}