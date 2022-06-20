use crate::LifeCycle;

use super::TypeInfo;

use thiserror::Error;

pub type BuildDependencyResult<T> = Result<T, BuildDependencyError>;

#[derive(Debug, Error)]
pub enum BuildDependencyError {
    #[error("Service not found {type_info:?}")]
    NotFound { type_info: TypeInfo },
    #[error("Service [{child_type_info:?}] resolve service [{parent_type_info:?}], which before resolve this service. If you app required cycled reference, remove loop-check feature")]
    CyclicReference { child_type_info: TypeInfo, parent_type_info: TypeInfo },
    #[error("Map component error. Idk how [{err:?}]")]
    MapComponentError { err: MapComponentError },
    #[error("Add component error. Probably you add service from ctr twice, or in other space and ctr second. Check service ctr [{err:?}]")]
    AddDependencyError { err: AddDependencyError },
    #[error("{err:?}")]
    Custom { err: anyhow::Error }
}

// Required for anyhow::Error compare
impl PartialEq for BuildDependencyError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NotFound { type_info: l_type_info }, Self::NotFound { type_info: r_type_info }) => l_type_info == r_type_info,
            (Self::CyclicReference { child_type_info: l_child_type_info, parent_type_info: l_parent_type_info }, Self::CyclicReference { child_type_info: r_child_type_info, parent_type_info: r_parent_type_info }) => l_child_type_info == r_child_type_info && l_parent_type_info == r_parent_type_info,
            (Self::MapComponentError { err: l_err }, Self::MapComponentError { err: r_err }) => l_err == r_err,
            (Self::AddDependencyError { err: l_err }, Self::AddDependencyError { err: r_err }) => l_err == r_err,
            (Self::Custom { err: _ }, Self::Custom { err: _ }) => true,
            _ => false,
        }
    }
}

pub type AddDependencyResult<T> = Result<T, AddDependencyError>;

#[derive(Debug, PartialEq, Error)]
pub enum AddDependencyError {
    #[error("Add component [{component_type_info:?}] error, component exist")]
    DependencyExist { component_type_info: TypeInfo, },   
}

pub type MapComponentResult<T> = Result<T, MapComponentError>;

#[derive(Debug, PartialEq, Error)]
pub enum MapComponentError {
    #[error("Map component [{component_type_info:?}] to service [{service_type_info:?}] error, component not found")]
    ComponentNotFound { component_type_info: TypeInfo, service_type_info: TypeInfo },
}

pub type DeleteComponentResult<T> = Result<T, DeleteComponentError>;

#[derive(Debug, PartialEq, Error)]
pub enum DeleteComponentError {
    #[error("Delete component [{component_type_info:?}] error, component not found")]
    ComponentNotFound { component_type_info: TypeInfo },
    #[error("Delete component [{component_type_info:?}] with life cycle [{life_cycle:?}] error, life cycle not support delete")]
    NotSupportedLifeCycle { component_type_info: TypeInfo, life_cycle: LifeCycle },
}