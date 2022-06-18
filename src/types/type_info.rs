use std::any::{TypeId, type_name};
use derive_new::new;

#[derive(Debug, Clone, new, PartialEq, Eq, Hash)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_name: String,
}

impl TypeInfo {
    pub fn from_type<TType: ?Sized + 'static>() -> Self {
        Self {
            type_id: TypeId::of::<TType>(),
            type_name: type_name::<TType>().into(), 
        }
    }
}